use std::path::PathBuf;

use crate::prelude::*;
use bevy::prelude::*;
use reactor_proto::prelude::*;

/// Plugin for all of `reactor_serial`. Add this to your app.
pub struct ReactorSerialPlugin {
    /// The root save path for the game.
    /// It is generally you're game name, with a default of `default_game`.
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
    pub root_save_path: PathBuf,
}

impl Plugin for ReactorSerialPlugin {
    fn build(&self, app: &mut App) {
        self.init_resources(app);
        self.add_systems(app);
        self.add_plugins(app);
    }
}

impl ReactorSerialPlugin {
    /// Creates a new `CerealPlugin` with a save root equal to the game name
    pub fn new(game_name: &str) -> Self {
        let game_name = game_name.replace(|c: char| !c.is_ascii(), "_");
        #[cfg(debug_assertions)]
        let mut path = PathBuf::from("./saves");
        #[cfg(not(debug_assertions))]
        let mut path = dirs::data_dir().unwrap();
        let game_path = PathBuf::from(game_name.clone());
        path.push(game_path);

        Self {
            root_save_path: path,
        }
    }

    fn init_resources(&self, app: &mut App) {
        app.insert_resource(RootSavePath(
            self.root_save_path.display().to_string().clone(),
        ))
        .insert_resource(CurrentSaveSlot(SaveSlot {
            name: "default_slot".to_string(),
        }))
        .insert_resource(SaveTasks(Vec::new()));
    }

    fn add_systems(&self, app: &mut App) {
        app.add_systems(Update, handle_load_task.in_set(CerealSchedule::Loading))
            .add_systems(Last, complete_save_tasks);
    }

    fn add_plugins(&self, app: &mut App) {
        app.add_plugins(ReactorProtoPlugin);
    }
}

impl Default for ReactorSerialPlugin {
    fn default() -> Self {
        Self::new("default_game")
    }
}
