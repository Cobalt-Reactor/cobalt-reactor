use bevy::prelude::*;

pub struct FocusPlugin;

impl Plugin for FocusPlugin {
    fn build(&self, app: &mut App) {
        self.add_events(app);
        self.add_plugins(app);
        self.register_types(app);
        self.insert_resources(app);
        self.add_systems(app);
        self.configure_sets(app);
    }
}

impl FocusPlugin {
    pub fn add_events(&self, _: &mut App) {}

    pub fn add_plugins(&self, _: &mut App) {}

    pub fn register_types(&self, _: &mut App) {}

    pub fn insert_resources(&self, _: &mut App) {}

    pub fn add_systems(&self, _: &mut App) {}

    pub fn configure_sets(&self, _: &mut App) {}
}
