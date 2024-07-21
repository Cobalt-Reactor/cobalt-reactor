use bevy::{ecs::event::event_update_system, prelude::*};
use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Mutex,
};

#[derive(Resource, Deref, DerefMut)]
pub struct ChannelReceiver<T>(Mutex<Receiver<T>>);

#[derive(Resource, Deref, DerefMut)]
pub struct ChannelSender<T>(pub Sender<T>);

pub trait EventChannelAppExt {
    // Allows you to create bevy events using mpsc Sender
    fn add_event_channel<T: Event>(&mut self) -> &mut Self;
}

impl EventChannelAppExt for App {
    fn add_event_channel<T: Event>(&mut self) -> &mut Self {
        let (sender, receiver) = channel::<T>();
        assert!(
            !self.world().contains_resource::<ChannelReceiver<T>>(),
            "this event channel is already initialized",
        );

        self.add_event::<T>();
        self.add_systems(First, channel_to_event::<T>.after(event_update_system));
        self.insert_resource(ChannelReceiver(Mutex::new(receiver)));
        self.insert_resource(ChannelSender(sender));
        self
    }
}

fn channel_to_event<T: Event>(receiver: Res<ChannelReceiver<T>>, mut writer: EventWriter<T>) {
    // this should be the only system working with the receiver,
    // thus we always expect to get this lock
    let events = receiver.lock().expect("unable to acquire mutex lock");
    writer.send_batch(events.try_iter());
}
