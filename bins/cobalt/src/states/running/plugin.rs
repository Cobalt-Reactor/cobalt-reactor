use super::systems::*;
use crate::{events::CloseCobalt, schedules::StateHandlingSchedule, CobaltState};
use bevy::prelude::*;
use reactor_core::perf_ui::ReactorPerfUiSchedule;

pub struct RunningStatePlugin;

impl Plugin for RunningStatePlugin {
    fn build(&self, app: &mut App) {
        self.add_systems(app);
        self.add_events(app);
        self.configure_sets(app);
    }
}

impl RunningStatePlugin {
    pub fn add_events(&self, app: &mut App) {
        app.add_event::<CloseCobalt>();
    }
    pub fn add_systems(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_close_cobalt, close_on_esc).in_set(StateHandlingSchedule::Running),
        );
    }

    pub fn configure_sets(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (StateHandlingSchedule::Running).run_if(in_state(CobaltState::Running)),
        );

        app.configure_sets(
            Update,
            (ReactorPerfUiSchedule::Spawn, ReactorPerfUiSchedule::Update)
                .run_if(in_state(CobaltState::Running)),
        );
    }
}
