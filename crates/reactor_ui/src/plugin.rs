use crate::sickle::SickleUiPlugin;
use bevy::prelude::*;

/// Plugin for all of `reactor_ui`. Add this to your app
#[derive(Default)]
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
    /// Creates a new `reactor_ui` with the given save root
    pub fn new() -> Self {
        Default::default()
    }

    fn add_events(&self, _: &mut App) {}

    fn add_plugins(&self, app: &mut App) {
        app.add_plugins(SickleUiPlugin);
    }

    fn register_types(&self, _: &mut App) {}

    fn insert_resources(&self, _: &mut App) {}

    fn add_systems(&self, _: &mut App) {}

    fn configure_sets(&self, _: &mut App) {}
}
