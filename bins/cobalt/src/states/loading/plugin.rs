use super::systems::print_progress_loading_game;
use crate::{schedules::StateHandlingSchedule, CobaltState};
use bevy::prelude::*;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt, LoadingStateSet};
use iyes_progress::prelude::ProgressPlugin;
use reactor_core::proto::ProtoSchedule;

pub struct LoadingStatePlugin;

impl Plugin for LoadingStatePlugin {
    fn build(&self, app: &mut App) {
        self.add_plugins(app);
        self.add_systems(app);
        self.configure_sets(app);
    }
}

impl LoadingStatePlugin {
    pub fn add_plugins(&self, app: &mut App) {
        app.add_plugins(
            ProgressPlugin::new(CobaltState::Loading)
                .continue_to(CobaltState::Running)
                .track_assets(),
        )
        .add_loading_state(
            LoadingState::new(CobaltState::Loading),
            //.load_collection::<ZombieAssets>(),
        );
    }

    pub fn add_systems(&self, app: &mut App) {
        app.add_systems(
            Update,
            (print_progress_loading_game).in_set(StateHandlingSchedule::Loading),
        );
    }

    pub fn configure_sets(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (ProtoSchedule::Loading, StateHandlingSchedule::Loading)
                .chain()
                .run_if(in_state(CobaltState::Loading))
                .after(LoadingStateSet(CobaltState::Loading)),
        );
    }
}
