use super::systems::*;
use crate::CobaltState;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        self.add_events(app);
        self.add_plugins(app);
        self.register_types(app);
        self.insert_resources(app);
        self.add_systems(app);
        self.configure_sets(app);
    }
}

impl UiPlugin {
    pub fn add_events(&self, _: &mut App) {}

    pub fn add_plugins(&self, _: &mut App) {}

    pub fn register_types(&self, _: &mut App) {}

    pub fn insert_resources(&self, _: &mut App) {}

    pub fn add_systems(&self, app: &mut App) {
        app.add_systems(OnEnter(CobaltState::Running), spawn_simple_widget);
    }

    pub fn configure_sets(&self, _: &mut App) {}
}
