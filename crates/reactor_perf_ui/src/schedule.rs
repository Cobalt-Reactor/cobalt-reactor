use bevy::prelude::*;

/// The schedule used by `reactor_perf_ui` to handle its systems.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ReactorPerfUiSchedule {
    /// Setting up widgets
    Setup,
    /// Spawning widgets
    Spawn,
    /// Updating widgets
    Update,
}
