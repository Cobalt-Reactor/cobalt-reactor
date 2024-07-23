use super::{grid::plugin::GridPlugin, systems::*};
use crate::{resources::*, CobaltState};
use bevy::prelude::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        self.add_plugins(app);
        self.register_types(app);
        self.insert_resources(app);
        self.add_systems(app);
    }
}

impl RenderPlugin {
    fn add_plugins(&self, app: &mut App) {
        app.add_plugins(GridPlugin::without_floor_grid());
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
}
