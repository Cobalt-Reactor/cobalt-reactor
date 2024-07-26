use bevy::prelude::*;
use reactor_ui::sickle::prelude::UiBuilder;

pub trait PerfUiEntry: Component {
    // Setup the entry. This is called once when the entry is added to the app.
    // Useful for loading resources, registering events, etc.
    fn setup(app: &mut App);

    // Spawn the entry
    // If you need any additional data, you can put Bevy system parameters
    // into `type SystemParamSpawn` and access them via `param`.
    //
    // Use the provided `commands` for spawning your entities.
    fn spawn(ui_builder: &mut UiBuilder<Entity>);
}
