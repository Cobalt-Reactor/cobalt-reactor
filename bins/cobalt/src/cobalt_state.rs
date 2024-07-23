use bevy::prelude::*;

/// Core app state
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum CobaltState {
    /// Loading state
    #[default]
    Loading,
    /// Running state
    Running,
    /// Exiting state
    Exiting,
}
