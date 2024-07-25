use bevy::prelude::*;

/// The main plugin for the Reactor Perf UI crate. Add this to your app to enable the Reactor Perf
/// UIs.
pub struct ReactorPerfUiPlugin;

impl Plugin for ReactorPerfUiPlugin {
    fn build(&self, app: &mut App) {
        self.add_events(app);
        self.add_plugins(app);
        self.register_types(app);
        self.insert_resources(app);
        self.add_systems(app);
        self.configure_sets(app);
    }
}

impl ReactorPerfUiPlugin {
    fn add_events(&self, _: &mut App) {}

    fn add_plugins(&self, _: &mut App) {}

    fn register_types(&self, _: &mut App) {}

    fn insert_resources(&self, _: &mut App) {}

    fn add_systems(&self, _: &mut App) {}

    fn configure_sets(&self, _: &mut App) {}
}
