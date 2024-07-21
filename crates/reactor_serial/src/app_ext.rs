use crate::prelude::*;
use bevy::prelude::*;

/// An extension trait for [`App`] to allow easy initialization of save data
pub trait CerealAppExt {
    /// Registers and sets up a save data type
    fn register_save_data<T>(&mut self) -> &mut Self
    where
        T: SaveData + 'static;
}

impl CerealAppExt for App {
    fn register_save_data<T>(&mut self) -> &mut Self
    where
        T: SaveData + 'static,
    {
        self.add_event::<SaveRequest<T>>()
            .add_event_channel::<SaveComplete<T>>()
            .add_event::<LoadRequest<T>>()
            .add_event_channel::<LoadComplete<T>>()
            .add_systems(
                Update,
                handle_save_request::<T>.in_set(CerealSchedule::Saving),
            )
            .add_systems(
                Update,
                handle_load_request::<T>.in_set(CerealSchedule::Loading),
            );

        self
    }
}
