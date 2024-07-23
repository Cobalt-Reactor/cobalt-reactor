use bevy::prelude::*;

/// Event to signal all cleanup is complete
#[derive(Debug, Event, Clone)]
pub(super) struct CleanupComplete;
