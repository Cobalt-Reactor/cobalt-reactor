use crate::prelude::*;
use reactor_proto::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// The version of the save format, must be monotonically increasing and start at 1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct SaveVersion(pub u32);

/// Trait for describing the format of a save type with defaults so you don't have to provide
/// implementations for every version.
pub trait SaveData:
    Clone + Debug + Send + Sync + Serialize + for<'a> Deserialize<'a> + 'static
{
    /// The `Prototype` this `SaveData` can convert to, defaults to `EmptyPrototype`
    type Output: Prototype + Serialize + for<'a> Deserialize<'a> = EmptyPrototype;

    /// The previous format of this `SaveData` or Self if this is the latest version
    /// defaults to Self
    type Previous: SaveData = TerminalSaveData;

    /// The next format of this `SaveData` or Self if this is the latest version
    /// defaults to Self
    type Next: SaveData = TerminalSaveData;

    /// The format of this `SaveData`
    const FORMAT: SaveFormat = SaveFormat::Ron;

    /// The current version of this `SaveData`, versions must monotonically increase
    const VERSION: SaveVersion;

    /// Converts the previous format into the current format returning
    /// the new format or None if the conversion is not possible, default implementation
    /// always returns None
    fn from_previous(_: &Self::Previous) -> Option<Self> {
        None
    }

    /// Converts the save data into a prototype, default implementation returns None
    fn to_prototype(&self) -> Option<Self::Output> {
        None
    }
}
