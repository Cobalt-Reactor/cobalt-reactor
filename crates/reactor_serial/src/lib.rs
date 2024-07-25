#![doc = include_str!("../README.md")]
#![feature(associated_type_defaults)]
mod app_ext;
mod commands_ext;
mod convert_save;
mod deserialize;
mod empty_prototype;
mod error;
mod event_channel;
mod events;
mod fs;
mod outcomes;
mod plugins;
mod resources;
mod save_data;
mod save_format;
mod schedule;
mod serialize;
mod systems;

/// Package contents
pub mod prelude {
    pub use crate::{
        app_ext::CerealAppExt,
        commands_ext::SaveSlotExt,
        convert_save::convert_save,
        empty_prototype::{EmptyPrototype, TerminalSaveData},
        error::CerealError,
        events::{Context, LoadComplete, LoadRequest, SaveComplete, SaveRequest},
        outcomes::{LoadOutcome, SaveOutcome},
        plugins::ReactorSerialPlugin,
        resources::{CurrentSaveSlot, CurrentUserID, RootSavePath, SaveSlot},
        save_data::{SaveData, SaveVersion},
        save_format::SaveFormat,
        schedule::CerealSchedule,
    };

    pub(crate) use crate::{
        deserialize::deserialize, event_channel::*, fs::check_create_folder, resources::SaveTasks,
        serialize::serialize, systems::*,
    };
}
