use bevy::prelude::*;

use crate::prelude::*;

pub struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut App) {
        self.add_events(app);
        self.add_plugins(app);
        self.register_types(app);
        self.insert_resources(app);
        self.add_systems(app);
        self.configure_sets(app);
    }
}

impl WidgetPlugin {
    fn add_events(&self, app: &mut App) {
        app.add_event::<CollapsibleClicked>();
    }

    fn add_plugins(&self, _: &mut App) {}

    fn register_types(&self, _: &mut App) {}

    fn insert_resources(&self, _: &mut App) {}

    fn add_systems(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_collapsible_clicked).in_set(ReactorUiSchedule::Update),
        );
    }

    fn configure_sets(&self, _: &mut App) {}
}
