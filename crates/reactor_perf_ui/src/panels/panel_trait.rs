use bevy::prelude::*;

/// Trait for the panels in the Reactor Perf UI.
pub trait ReactorPerfUiPanel: Component + Default {
    /// Configuration type for the panel
    type Config: Resource + Clone;

    /// Setup the widget. This is called once when the widget is added to the app.
    /// Useful for loading resources, registering events, etc.
    fn setup(app: &mut App, config: Self::Config);

    /// Function that spawns the widget
    fn spawn(commands: Commands, config: Res<Self::Config>);
}
