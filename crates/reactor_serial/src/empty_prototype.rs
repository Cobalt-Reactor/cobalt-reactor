use crate::prelude::*;
use reactor_proto::prelude::*;
use serde::{Deserialize, Serialize};

/// An empty prototype
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyPrototype;

impl Prototype for EmptyPrototype {
    fn name(&self) -> String {
        String::new()
    }

    fn build(&self, _: &mut bevy::prelude::EntityWorldMut) {}

    fn rebuild(&self, _: &mut bevy::prelude::EntityWorldMut) {}
}

/// A struct representing the top most or bottom most save data
/// in the save data hierarchy
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSaveData;

impl SaveData for TerminalSaveData {
    type Output = EmptyPrototype;
    const VERSION: SaveVersion = SaveVersion(0);

    fn from_previous(_: &Self::Previous) -> Option<Self> {
        None
    }

    fn to_prototype(&self) -> Option<Self::Output> {
        None
    }
}
