use bevy::prelude::*;

/// Plugin for all of `reactor_core`. To setup all it's sub-plugins, add this to your app.
pub struct ReactorCorePlugin {
    /// The name of your game, used to create the root save path for `rantz_serial`.
    /// This folder will be created in the users data directory in release builds:
    ///
    /// Platform  | Value                                   | Example
    /// Linux     | `$XDG_DATA_HOME` or $HOME/.local/share  | /home/alice/.local/share
    /// macOS     | $HOME/Library/Application Support       | /Users/Alice/Library/Application
    /// Support Windows   | `{FOLDERID_RoamingAppData}`             |
    /// C:\Users\Alice\AppData\Roaming
    ///
    /// In debug builds, the folder will be created in `saves/` in the workspace root.
    ///
    /// any non-ascii characters in the name will be replaced with _
    #[cfg(feature = "serial")]
    pub game_name: String,
}

impl Plugin for ReactorCorePlugin {
    fn build(&self, app: &mut App) {
        self.init_resources(app);
        self.add_systems(app);
        self.add_plugins(app);
    }
}

impl ReactorCorePlugin {
    /// Creates a new `serialPlugin` with the given save root
    pub fn new(root_save_path: &str) -> Self {
        let root_save_path = root_save_path.replace(|c: char| !c.is_ascii(), "_");
        Self {
            game_name: root_save_path,
        }
    }

    fn init_resources(&self, _: &mut App) {}

    fn add_systems(&self, _: &mut App) {}

    fn add_plugins(&self, app: &mut App) {
        #[cfg(feature = "spatial")]
        app.add_plugins(crate::spatial::ReactorSpatialPlugin::new());
        #[cfg(feature = "camera")]
        app.add_plugins(crate::camera::ReactorCameraPlugin::new());
        #[cfg(all(feature = "proto", not(feature = "serial")))]
        app.add_plugins(crate::proto::ReactorProtoPlugin::new());
        #[cfg(feature = "serial")]
        app.add_plugins(crate::serial::ReactorSerialPlugin::new(&self.game_name));
        #[cfg(feature = "ui")]
        app.add_plugins(crate::ui::ReactorUiPlugin::new());
        #[cfg(feature = "perf_ui")]
        app.add_plugins(
            crate::perf_ui::ReactorPerfUiPlugin::new()
                .start_visible()
                .with_perf_panel_config(crate::perf_ui::PerfPanelConfig::full()),
        );
    }
}

impl Default for ReactorCorePlugin {
    fn default() -> Self {
        Self::new("default_game")
    }
}
