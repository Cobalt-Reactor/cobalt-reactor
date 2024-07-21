mod shared;
use bevy::{ecs::world::CommandQueue, prelude::*};
use reactor_serial::prelude::*;
use shared::*;

#[test]
fn set_user_id() {
    let mut world = World::new();
    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, &world);
    commands.set_user_id("test_user");
    queue.apply(&mut world);

    let id = world.get_resource::<CurrentUserID>().unwrap();
    assert_eq!(id.id, "test_user".to_string());
}

#[test]
fn set_current_save_slot() {
    let mut app = App::new();
    app.add_plugins(TaskPoolPlugin::default())
        .add_plugins(SerialPlugin::new("test_game"))
        .register_save_data::<TestSaveFormatV3>();

    let world = app.world_mut();
    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, world);
    commands.set_save_slot("test_slot");
    queue.apply(world);

    let slot = world.get_resource::<CurrentSaveSlot>().unwrap();
    assert_eq!(slot.name, "test_slot");
}
