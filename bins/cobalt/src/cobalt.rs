use crate::{
    defines::{logging_defines, window_defines},
    plugins::*,
    CobaltState,
};
use bevy::{log::LogPlugin, prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use reactor_core::ReactorCorePlugin;

/// Core game struct
pub struct Cobalt {
    /// Bevy app
    app: App,
}

impl Cobalt {
    /// Create a new game
    pub fn new() -> Self {
        Default::default()
    }

    /// Runs the game
    pub fn run(&mut self) {
        self.app.run();
    }
}

impl Default for Cobalt {
    fn default() -> Self {
        let mut app = App::new();

        app.add_plugins((DefaultPlugins
            .set(LogPlugin {
                filter: logging_defines::DEFAULT_FILTER.into(),
                level: logging_defines::DEFAULT_LEVEL,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from(window_defines::TITLE),
                    resolution: WindowResolution::from(window_defines::RESOLUTION),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),))
            .init_state::<CobaltState>()
            // External plugins
            .add_plugins(ReactorCorePlugin::new("cobalt"))
            // Internal plugins
            .add_plugins(StatesPlugin)
            .add_plugins(UiPlugin)
            .add_plugins(RenderPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugins(DebugPlugin)
                .add_plugins(WorldInspectorPlugin::new());
        }

        Self { app }
    }
}
