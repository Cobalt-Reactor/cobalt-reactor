use std::{fs, thread::sleep};

use bevy::prelude::*;
use reactor_proto::prelude::*;
use reactor_serial::prelude::*;
use serde::{Deserialize, Serialize};

// Allowed because for some reason Rust Analyzer thinks this is dead code when it isn't
#[allow(dead_code)]
pub fn check_file_exists(path: &str) -> bool {
    sleep(std::time::Duration::from_millis(100)); // Give the OS time to do the thing
    std::path::Path::new(path).exists()
}

// Allowed because for some reason Rust Analyzer thinks this is dead code when it isn't
#[allow(dead_code)]
pub fn delete_folder(path: &str) {
    if std::path::Path::new(path).exists() {
        fs::remove_dir_all(path).unwrap();
    }
}

// RON
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct TestPrototype {
    pub name: String,
    pub value: f32,
}

impl Prototype for TestPrototype {
    fn name(&self) -> String {
        format!("TestPrototype:{}", self.name)
    }

    fn build(&self, _: &mut bevy::prelude::EntityWorldMut) {}
    fn rebuild(&self, _: &mut bevy::prelude::EntityWorldMut) {}
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TestSaveFormatV3 {
    pub name: String,
    pub value: f32,
}

impl SaveData for TestSaveFormatV3 {
    type Output = TestPrototype;
    type Previous = TestSaveFormatV2;
    const VERSION: SaveVersion = SaveVersion(3);

    fn from_previous(previous: &Self::Previous) -> Option<Self> {
        Some(TestSaveFormatV3 {
            name: previous.name.clone(),
            value: previous.value as f32,
        })
    }

    fn to_prototype(&self) -> Option<Self::Output> {
        Some(TestPrototype {
            name: self.name.clone(),
            value: self.value,
        })
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TestSaveFormatV2 {
    pub name: String,
    pub value: u32,
}

impl SaveData for TestSaveFormatV2 {
    type Previous = TestSaveFormatV1;
    type Next = TestSaveFormatV3;
    const VERSION: SaveVersion = SaveVersion(2);

    fn from_previous(previous: &Self::Previous) -> Option<Self> {
        Some(TestSaveFormatV2 {
            name: previous.name.clone(),
            value: 0,
        })
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TestSaveFormatV1 {
    pub name: String,
}

impl SaveData for TestSaveFormatV1 {
    type Next = TestSaveFormatV2;
    const VERSION: SaveVersion = SaveVersion(1);
}

// JSON
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TestSaveFormatV2Json {
    pub name: String,
    pub value: f32,
}

impl SaveData for TestSaveFormatV2Json {
    type Output = TestPrototype;
    type Previous = TestSaveFormatV1Json;
    const VERSION: SaveVersion = SaveVersion(3);
    const FORMAT: SaveFormat = SaveFormat::Json;

    fn from_previous(previous: &Self::Previous) -> Option<Self> {
        Some(TestSaveFormatV2Json {
            name: previous.name.clone(),
            value: 0.0,
        })
    }

    fn to_prototype(&self) -> Option<Self::Output> {
        Some(TestPrototype {
            name: self.name.clone(),
            value: self.value,
        })
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TestSaveFormatV1Json {
    pub name: String,
}

impl SaveData for TestSaveFormatV1Json {
    type Next = TestSaveFormatV2Json;
    const VERSION: SaveVersion = SaveVersion(1);
    const FORMAT: SaveFormat = SaveFormat::Json;
}

// YAML
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TestSaveFormatV2Yaml {
    pub name: String,
    pub value: f32,
}

impl SaveData for TestSaveFormatV2Yaml {
    type Output = TestPrototype;
    type Previous = TestSaveFormatV1Yaml;
    const VERSION: SaveVersion = SaveVersion(3);
    const FORMAT: SaveFormat = SaveFormat::Yaml;

    fn from_previous(previous: &Self::Previous) -> Option<Self> {
        Some(TestSaveFormatV2Yaml {
            name: previous.name.clone(),
            value: 0.0,
        })
    }

    fn to_prototype(&self) -> Option<Self::Output> {
        Some(TestPrototype {
            name: self.name.clone(),
            value: self.value,
        })
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TestSaveFormatV1Yaml {
    pub name: String,
}

impl SaveData for TestSaveFormatV1Yaml {
    type Next = TestSaveFormatV2Yaml;
    const VERSION: SaveVersion = SaveVersion(1);
    const FORMAT: SaveFormat = SaveFormat::Yaml;
}

// TOML
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TestSaveFormatV2Toml {
    pub name: String,
    pub value: f32,
}

impl SaveData for TestSaveFormatV2Toml {
    type Output = TestPrototype;
    type Previous = TestSaveFormatV1Toml;
    const VERSION: SaveVersion = SaveVersion(3);
    const FORMAT: SaveFormat = SaveFormat::Toml;

    fn from_previous(previous: &Self::Previous) -> Option<Self> {
        Some(TestSaveFormatV2Toml {
            name: previous.name.clone(),
            value: 0.0,
        })
    }

    fn to_prototype(&self) -> Option<Self::Output> {
        Some(TestPrototype {
            name: self.name.clone(),
            value: self.value,
        })
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TestSaveFormatV1Toml {
    pub name: String,
}

impl SaveData for TestSaveFormatV1Toml {
    type Next = TestSaveFormatV2Toml;
    const VERSION: SaveVersion = SaveVersion(1);
    const FORMAT: SaveFormat = SaveFormat::Toml;
}

// XML
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TestSaveFormatV2Xml {
    pub name: String,
    pub value: f32,
}

impl SaveData for TestSaveFormatV2Xml {
    type Output = TestPrototype;
    type Previous = TestSaveFormatV1Xml;
    const VERSION: SaveVersion = SaveVersion(3);
    const FORMAT: SaveFormat = SaveFormat::Xml;

    fn from_previous(previous: &Self::Previous) -> Option<Self> {
        Some(TestSaveFormatV2Xml {
            name: previous.name.clone(),
            value: 0.0,
        })
    }

    fn to_prototype(&self) -> Option<Self::Output> {
        Some(TestPrototype {
            name: self.name.clone(),
            value: self.value,
        })
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TestSaveFormatV1Xml {
    pub name: String,
}

impl SaveData for TestSaveFormatV1Xml {
    type Next = TestSaveFormatV2Xml;
    const VERSION: SaveVersion = SaveVersion(1);
    const FORMAT: SaveFormat = SaveFormat::Xml;
}

// CSV
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TestSaveFormatV2Csv {
    pub name: String,
    pub value: f32,
}

impl SaveData for TestSaveFormatV2Csv {
    type Output = TestPrototype;
    type Previous = TestSaveFormatV1Csv;
    const VERSION: SaveVersion = SaveVersion(3);
    const FORMAT: SaveFormat = SaveFormat::Csv;

    fn from_previous(previous: &Self::Previous) -> Option<Self> {
        Some(TestSaveFormatV2Csv {
            name: previous.name.clone(),
            value: 0.0,
        })
    }

    fn to_prototype(&self) -> Option<Self::Output> {
        Some(TestPrototype {
            name: self.name.clone(),
            value: self.value,
        })
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TestSaveFormatV1Csv {
    pub name: String,
}

impl SaveData for TestSaveFormatV1Csv {
    type Next = TestSaveFormatV2Csv;
    const VERSION: SaveVersion = SaveVersion(1);
    const FORMAT: SaveFormat = SaveFormat::Csv;
}

// MsgPack
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TestSaveFormatV2MsgPack {
    pub name: String,
    pub value: f32,
}

impl SaveData for TestSaveFormatV2MsgPack {
    type Output = TestPrototype;
    type Previous = TestSaveFormatV1MsgPack;
    const VERSION: SaveVersion = SaveVersion(3);
    const FORMAT: SaveFormat = SaveFormat::MsgPack;

    fn from_previous(previous: &Self::Previous) -> Option<Self> {
        Some(TestSaveFormatV2MsgPack {
            name: previous.name.clone(),
            value: 0.0,
        })
    }

    fn to_prototype(&self) -> Option<Self::Output> {
        Some(TestPrototype {
            name: self.name.clone(),
            value: self.value,
        })
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TestSaveFormatV1MsgPack {
    pub name: String,
}

impl SaveData for TestSaveFormatV1MsgPack {
    type Next = TestSaveFormatV2MsgPack;
    const VERSION: SaveVersion = SaveVersion(1);
    const FORMAT: SaveFormat = SaveFormat::MsgPack;
}

// Cyclical Reference
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CyclicalReferenceV1;

impl SaveData for CyclicalReferenceV1 {
    type Next = CyclicalReferenceV2;
    type Previous = CyclicalReferenceV2;
    const VERSION: SaveVersion = SaveVersion(1);
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CyclicalReferenceV2;

impl SaveData for CyclicalReferenceV2 {
    type Next = CyclicalReferenceV1;
    type Previous = CyclicalReferenceV1;
    const VERSION: SaveVersion = SaveVersion(2);
}

// Matching Save Versions
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct MatchingSaveVersionsV1;

impl SaveData for MatchingSaveVersionsV1 {
    type Next = MatchingSaveVersionsV2;
    const VERSION: SaveVersion = SaveVersion(1);
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct MatchingSaveVersionsV2;

impl SaveData for MatchingSaveVersionsV2 {
    type Previous = MatchingSaveVersionsV1;
    const VERSION: SaveVersion = SaveVersion(1);
}

// Decremented Save Versions
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct DecrementedSaveVersionsV1;

impl SaveData for DecrementedSaveVersionsV1 {
    type Next = DecrementedSaveVersionsV2;
    const VERSION: SaveVersion = SaveVersion(2);
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct DecrementedSaveVersionsV2;

impl SaveData for DecrementedSaveVersionsV2 {
    type Previous = DecrementedSaveVersionsV1;
    const VERSION: SaveVersion = SaveVersion(1);
}

// Decremented Save Versions
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct BadFormat {
    pub bad: i32,
}

impl SaveData for BadFormat {
    const VERSION: SaveVersion = SaveVersion(1);
}
