use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum StateHandlingSchedule {
    Loading,
    Running,
    Exiting,
}
