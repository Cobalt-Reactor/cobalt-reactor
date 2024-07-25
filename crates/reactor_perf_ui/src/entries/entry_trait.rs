use bevy::{ecs::system::SystemParam, prelude::*};
use reactor_ui::sickle::prelude::UiBuilder;

pub trait PerfUiEntry: Component {
    /// Any system parameters you need to update the UI.
    type SystemParamUpdate: SystemParam + 'static;

    // Setup the entry. This is called once when the entry is added to the app.
    // Useful for loading resources, registering events, etc.
    fn setup(app: &mut App);

    // Spawn the entry
    // If you need any additional data, you can put Bevy system parameters
    // into `type SystemParamSpawn` and access them via `param`.
    //
    // Use the provided `commands` for spawning your entities.
    fn spawn(ui_builder: &mut UiBuilder<Entity>);

    // Update the entry.
    //
    // You can use arbitrary Bevy system parameters to access the data
    // you need to update the UI. Put them in `type SystemParamUpdate`
    // and access them via `param`.
    fn update(entity: Entity, param: &mut <Self::SystemParamUpdate as SystemParam>::Item<'_, '_>);

    /// The sort key of the entry that the widget is displaying.
    fn sort_key(&self) -> i32;
}
