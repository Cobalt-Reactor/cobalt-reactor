use super::{events::*, systems::*};
use crate::{schedules::StateHandlingSchedule, CobaltState};
use bevy::prelude::*;

pub struct ExitingStatePlugin;

impl Plugin for ExitingStatePlugin {
    fn build(&self, app: &mut App) {
        self.add_systems(app);
        self.add_events(app);
        self.configure_sets(app);
    }
}

impl ExitingStatePlugin {
    pub fn add_events(&self, app: &mut App) {
        app.add_event::<CleanupComplete>();
    }

    pub fn add_systems(&self, app: &mut App) {
        app.add_systems(
            Update,
            (signal_cleanup_complete, exit_app).in_set(StateHandlingSchedule::Exiting),
        );
    }

    pub fn configure_sets(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (StateHandlingSchedule::Exiting).run_if(in_state(CobaltState::Exiting)),
        );
    }
}
