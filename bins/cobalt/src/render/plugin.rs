use super::{
    focus::plugin::FocusPlugin, grid::plugin::GridPlugin, panning::plugin::CameraPanningPlugin,
    systems::*,
};
use crate::{resources::*, CobaltState};
use bevy::prelude::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        self.add_events(app);
        self.add_plugins(app);
        self.register_types(app);
        self.insert_resources(app);
        self.add_systems(app);
        self.configure_sets(app);
    }
}

impl RenderPlugin {
    pub fn add_events(&self, _: &mut App) {}

    fn add_plugins(&self, app: &mut App) {
        app.add_plugins(GridPlugin::without_floor_grid())
            .add_plugins(CameraPanningPlugin)
            .add_plugins(FocusPlugin);
    }

    fn register_types(&self, _: &mut App) {}

    fn insert_resources(&self, app: &mut App) {
        app.insert_resource(Msaa::Off)
            .insert_resource(ClearColor(Color::BLACK))
            .insert_resource(MouseWorldCoords::default());
    }

    fn add_systems(&self, app: &mut App) {
        app.add_systems(OnEnter(CobaltState::Running), (spawn_camera, create_grid))
            .add_systems(
                Update,
                (mouse_to_world).run_if(in_state(CobaltState::Running)),
            );
    }

    pub fn configure_sets(&self, _: &mut App) {}
}
