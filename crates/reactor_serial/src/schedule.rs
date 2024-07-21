use bevy::prelude::*;

/// The schedule for loading and spawning prototypes
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CerealSchedule {
    /// The schedule for handling load requests. If you want to control when
    /// loading happens, you can `configure_sets` on this schedule
    Loading,
    /// The schedule for handling save requests. If you want to control when
    /// saving happens, you can `configure_sets` on this schedule
    Saving,
}
