use crate::prelude::*;
use arrayref::array_ref;
use bevy::{
    ecs::world::CommandQueue,
    prelude::*,
    tasks::{block_on, poll_once, AsyncComputeTaskPool, IoTaskPool, Task},
};
use reactor_proto::prelude::SpawnPrototypeExt;
use std::{
    any::type_name,
    fs::{self, File},
    future::IntoFuture,
    io::Write,
    marker::PhantomData,
    path::PathBuf,
};

pub fn complete_save_tasks(mut tasks: ResMut<SaveTasks>, exit_reader: EventReader<AppExit>) {
    if exit_reader.is_empty() {
        tasks.retain(|task| !task.is_finished());
        return;
    }

    tasks
        .drain(..)
        .for_each(|task| block_on(task.into_future()));
}

pub fn handle_save_request<T>(
    mut save_request_reader: EventReader<SaveRequest<T>>,
    root_save_path: Res<RootSavePath>,
    save_slot: Res<CurrentSaveSlot>,
    user_id: Option<Res<CurrentUserID>>,
    mut task_list: ResMut<SaveTasks>,
    channel_sender: Res<ChannelSender<SaveComplete<T>>>,
) where
    T: SaveData,
{
    for request in save_request_reader.read() {
        let root_save_path = root_save_path.clone();
        let mut moveable_id = None;
        if let Some(id) = &user_id {
            moveable_id = Some((**id).clone());
        }
        let save_slot = save_slot.clone();
        let request = request.clone();
        let sender = channel_sender.clone();

        let task = IoTaskPool::get().spawn(async move {
            match write_to_disk(&root_save_path, &moveable_id, &save_slot, &request) {
                Ok(_) => {
                    let _ = sender.send(SaveComplete {
                        outcome: SaveOutcome::Success,
                        file_name: request.file_name.clone(),
                        _marker: PhantomData,
                    });
                }
                Err(e) => {
                    let _ = sender.send(SaveComplete {
                        outcome: SaveOutcome::Failure(e),
                        file_name: request.file_name,
                        _marker: PhantomData,
                    });
                }
            }
        });
        task_list.push(task);
    }
}

#[derive(Component, Debug)]
pub struct LoadTask(pub Task<CommandQueue>);

pub fn handle_load_task(mut commands: Commands, mut load_tasks: Query<(Entity, &mut LoadTask)>) {
    for (e, mut load_task) in load_tasks.iter_mut() {
        if load_task.0.is_finished() {
            if let Some(mut command_queue) = block_on(poll_once(&mut load_task.0)) {
                commands.append(&mut command_queue);
                commands.entity(e).despawn_recursive();
            }
        }
    }
}

pub fn handle_load_request<T>(
    mut commands: Commands,
    mut load_request_reader: EventReader<LoadRequest<T>>,
    root_save_path: Res<RootSavePath>,
    save_slot: Res<CurrentSaveSlot>,
    user_id: Option<Res<CurrentUserID>>,
    channel_sender: Res<ChannelSender<LoadComplete<T>>>,
) where
    T: SaveData,
{
    let thread_pool = AsyncComputeTaskPool::get();
    for request in load_request_reader.read() {
        let root_save_path = root_save_path.clone();
        let mut moveable_id = None;
        if let Some(id) = &user_id {
            moveable_id = Some((**id).clone());
        }
        let save_slot = save_slot.clone();
        let request = request.clone();
        let sender = channel_sender.0.clone();

        let task = thread_pool.spawn(async move {
            let mut queue = CommandQueue::default();
            let result = match read_from_disk(&root_save_path, &moveable_id, &save_slot, &request) {
                Ok(result) => result,
                Err(e) => {
                    let _ = sender.send(LoadComplete::<T> {
                        outcome: LoadOutcome::Failure(e.into()),
                        file_name: request.file_name.clone(),
                        _marker: PhantomData,
                    });
                    return queue;
                }
            };

            let bytes = result.as_bytes();
            let v_bytes = array_ref![bytes, 0, 4];
            let version = SaveVersion(u32::from_be_bytes(*v_bytes));
            let data = &bytes[4..];

            let output = match convert_save::<T>(version, &request.file_name, data) {
                Ok(output) => output,
                Err(e) => {
                    let _ = sender.send(LoadComplete::<T> {
                        outcome: LoadOutcome::Failure(e),
                        file_name: request.file_name.clone(),
                        _marker: PhantomData,
                    });
                    return queue;
                }
            };

            let Some(prototype) = output.to_prototype() else {
                let _ = sender.send(LoadComplete::<T> {
                    outcome: LoadOutcome::Failure(CerealError::PrototypeConversionFailed {
                        prototype_name: type_name::<T::Output>().to_string(),
                        type_name: type_name::<T>().to_string(),
                    }),
                    file_name: request.file_name.clone(),
                    _marker: PhantomData,
                });
                return queue;
            };

            queue.spawn_prototype(prototype);

            let _ = sender.send(LoadComplete::<T> {
                outcome: LoadOutcome::Success,
                file_name: request.file_name.clone(),
                _marker: PhantomData,
            });

            queue
        });

        commands
            .spawn_empty()
            .insert(LoadTask(task))
            .insert(Name::new("Cereal Load Task"));
    }
}

fn read_from_disk<T>(
    root_save_path: &RootSavePath,
    moveable_id: &Option<CurrentUserID>,
    save_slot: &CurrentSaveSlot,
    request: &LoadRequest<T>,
) -> Result<String, std::io::Error>
where
    T: SaveData,
{
    let mut path = build_directory_name(
        root_save_path,
        moveable_id,
        save_slot,
        &request.sub_directory,
        &request.context,
    );
    path.push(&request.file_name);
    fs::read_to_string(path)
}

fn write_to_disk<T>(
    root_save_path: &RootSavePath,
    moveable_id: &Option<CurrentUserID>,
    save_slot: &CurrentSaveSlot,
    request: &SaveRequest<T>,
) -> Result<(), CerealError>
where
    T: SaveData,
{
    let mut path = build_directory_name(
        root_save_path,
        moveable_id,
        save_slot,
        &request.sub_directory,
        &request.context,
    );
    check_create_folder(&path)?;
    path.push(&request.file_name);

    let output = serialize(&request.save_data)?;

    let mut file = File::create(path)?;
    file.write_all(&T::VERSION.0.to_be_bytes())?;
    file.write_all(&output)?;

    Ok(())
}

fn build_directory_name(
    root_save_path: &RootSavePath,
    user_id: &Option<CurrentUserID>,
    save_slot: &CurrentSaveSlot,
    sub_directory: &Option<String>,
    context: &Context,
) -> PathBuf {
    let mut path = PathBuf::from(root_save_path.0.clone());

    match context {
        Context::Global => path,
        Context::User => {
            if let Some(user_id) = user_id {
                path.push(user_id.id.clone());
            }
            path
        }
        Context::Slot => {
            if let Some(id) = user_id {
                path.push(id.id.clone());
            }

            path.push(save_slot.name.clone());

            if let Some(sub_directory) = sub_directory {
                path.push(sub_directory.clone());
            }

            path
        }
    }
}
