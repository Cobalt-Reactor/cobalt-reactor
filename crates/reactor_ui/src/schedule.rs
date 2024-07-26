use bevy::prelude::*;

/// The schedule used by `reactor__ui` to handle its systems.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ReactorUiSchedule {
    /// Updating widgets
    Update,
}
