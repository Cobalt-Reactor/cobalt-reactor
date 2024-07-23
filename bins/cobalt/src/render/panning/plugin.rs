use super::{components::*, systems::*};
use crate::CobaltState;
use bevy::prelude::*;

pub struct CameraPanningPlugin;

impl Plugin for CameraPanningPlugin {
    fn build(&self, app: &mut App) {
        self.add_events(app);
        self.add_plugins(app);
        self.register_types(app);
        self.add_systems(app);
        self.configure_sets(app);
    }
}

impl CameraPanningPlugin {
    pub fn add_events(&self, _: &mut App) {}

    pub fn add_plugins(&self, _: &mut App) {}

    pub fn register_types(&self, app: &mut App) {
        app.register_type::<PanCam>();
    }
    pub fn add_systems(&self, app: &mut App) {
        app.add_systems(
            Update,
            (camera_movement, camera_zoom).run_if(in_state(CobaltState::Running)),
        );
    }

    pub fn configure_sets(&self, _: &mut App) {}
}
