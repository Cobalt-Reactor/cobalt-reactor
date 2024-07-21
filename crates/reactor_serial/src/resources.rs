use bevy::{prelude::*, tasks::Task};

/// The root save path for all save data.
/// It is generally you're game name, with a default of `default_game`.
/// This folder will be created in the users data directory in release builds:
///
/// Platform  | Value                                   | Example
/// Linux     | `$XDG_DATA_HOME` or $HOME/.local/share  | /home/alice/.local/share
/// macOS     | $HOME/Library/Application Support       | /Users/Alice/Library/Application Support
/// Windows   | `{FOLDERID_RoamingAppData}`             | C:\Users\Alice\AppData\Roaming
///
/// In debug builds, the folder will be created in `saves/` in the workspace root.
///
/// any non-ascii characters in the name will be replaced with _
#[derive(Resource, Debug, PartialEq, Eq, Reflect, Clone, Deref, DerefMut)]
pub struct RootSavePath(pub(crate) String);

/// The definition of a save slot
#[derive(Reflect, Debug, PartialEq, Eq, Clone)]
pub struct SaveSlot {
    /// The name of the save slot, matches the folder name in the save path
    pub name: String,
}

/// The current save slot folder name, saved data will be stored relative to it
#[derive(Resource, Reflect, Debug, PartialEq, Eq, Clone, Deref, DerefMut)]
pub struct CurrentSaveSlot(pub SaveSlot);

/// The current user id, an optional folder name for save save slots to live under
/// for when your game needs to store files that are user specific (i.e. steam ids)
#[derive(Resource, Reflect, Debug, PartialEq, Eq, Clone, Deref, DerefMut)]
pub struct CurrentUserID {
    /// The id of the user as a string
    pub id: String,
}

#[derive(Default, Debug, Resource, Deref, DerefMut)]
pub struct SaveTasks(pub Vec<Task<()>>);
