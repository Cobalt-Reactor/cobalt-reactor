use crate::prelude::*;
use thiserror::Error;

/// An error that can occur during saving or loading
#[derive(Error, Debug, Clone)]
pub enum CerealError {
    /// Failed to deserialize the save data as a RON file
    /// Wraps `ron::Error`
    #[cfg(feature = "ron")]
    #[error("RON deserialization failed: {0}")]
    RonSerializationFailed(#[from] ron::Error),

    /// Failed to serialize the save data as a RON file
    /// Wraps `ron::Error`
    #[cfg(feature = "ron")]
    #[error("RON Serialization failed: {0}")]
    RonDeserializationFailed(#[from] ron::de::SpannedError),

    /// Failed to serialize/deserialize the save data as a JSON file
    /// Wraps `serde_json::Error`
    #[cfg(feature = "json")]
    #[error("JSON save/load failed: {error:?}")]
    JsonSaveLoadFailed {
        /// The error message
        error: String,
    },

    /// Failed to serialize/deserialize the save data as a YAML file
    /// Wraps `serde_yaml::Error`
    #[cfg(feature = "yaml")]
    #[error("YAML save/load failed: {error:?}")]
    YamlSaveLoadFailed {
        /// The error message
        error: String,
    },

    /// Failed to serialize the save data as a TOML file
    /// Wraps `toml::ser::Error`
    #[cfg(feature = "toml")]
    #[error("TOML serialization failed: {0}")]
    TomlSaveFailed(#[from] toml::ser::Error),

    /// Failed to deserialize the save data as a TOML file
    /// Wraps `toml::Error`
    #[cfg(feature = "toml")]
    #[error("TOML deserialization failed: {0}")]
    TomlLoadFailed(#[from] toml::de::Error),

    /// Failed to deserialize the save data as the contents are
    /// not valid UTF-8. Wraps `std::str::Utf8Error`
    #[cfg(feature = "toml")]
    #[error("UTF-8 read failed: {0}")]
    Utf8ReadFailed(#[from] std::str::Utf8Error),

    /// Failed to serialize/deserialize the save data as a XML file
    /// Wraps `quick_xml::DeError`
    #[cfg(feature = "xml")]
    #[error("XML save/load failed: {0}")]
    XmlSaveLoadFailed(#[from] quick_xml::DeError),

    /// Failed to serialize/deserialize the save data as a XML file
    /// Wraps `csv::Error`
    #[cfg(feature = "csv")]
    #[error("CSV save/load failed: {error:?}")]
    CsvSaveLoadFailed {
        /// The error message
        error: String,
    },

    /// Failed to deserialize the save data as a XML file
    /// Wraps `rmp_serde::decode::Error`
    #[cfg(feature = "msgpack")]
    #[error("MsgPack save/load failed: {error:?}")]
    MsgPackSaveLoadFailed {
        /// The error message
        error: String,
    },

    /// Failed to read save data file
    #[error("Failed to read save data file: {error:?}")]
    IoError {
        /// The error message
        error: String,
    },

    /// Failed to deserialize the save data as the file format
    /// did not match the expected format
    #[error("Failed to convert: {file_name:?} as format: {format:?}")]
    FormatMismatch {
        /// The name of the save file
        file_name: String,
        /// The expected format
        format: SaveFormat,
    },

    /// Failed to deserialize the save data, because we had to walk the
    /// version chain to find the correct implementation and there was
    /// a duplicate version in the chain
    #[error(
        "Failed to convert: {file_name:?} as SaveData. Version chain \
    has multiple implementations with the same SaveVersion"
    )]
    VersionChainDuplicate {
        /// The name of the save file
        file_name: String,
    },

    /// Failed to deserialize the save data, because we had to walk the
    /// version chain to find the correct implementation and there was
    /// a decremented version in the chain above the current version
    /// ie. CURRENT = 1, PREVIOUS = 2
    #[error(
        "Failed to convert: {file_name:?} as SaveData. Version chain \
    had a Next Version with a decremented version number"
    )]
    VersionChainDecrement {
        /// The name of the save file
        file_name: String,
    },

    /// Failed to upgrade a save version using `from_previous`
    #[error("Failed to upgrade version: {file_name:?} from: {from:?} to: {to:?}")]
    VersionUpgradeFailed {
        /// The name of the save file
        file_name: String,
        /// The name of the type being upgraded from
        from: String,
        /// The name of the type being upgraded to
        to: String,
    },

    /// Failed to convert a save data that was successfully deserialized
    /// into it's associated prototype
    #[error("Failed to convert: {type_name:?} to: {prototype_name:?}")]
    PrototypeConversionFailed {
        /// The name of the save file
        prototype_name: String,
        /// The name of the type being upgraded from
        type_name: String,
    },

    /// Failed to upgrade a save version using `from_previous`
    #[error(
        "Failed to deserialize: {file_name:?} all options failed for type: {to:?}, reached the terminal version."
    )]
    NoOptionInChain {
        /// The name of the save file
        file_name: String,
        /// The name of the type being upgraded to
        to: String,
    },
}

impl From<std::io::Error> for CerealError {
    fn from(error: std::io::Error) -> Self {
        CerealError::IoError {
            error: error.to_string(),
        }
    }
}

#[cfg(feature = "msgpack")]
impl From<rmp_serde::decode::Error> for CerealError {
    fn from(error: rmp_serde::decode::Error) -> Self {
        CerealError::MsgPackSaveLoadFailed {
            error: error.to_string(),
        }
    }
}

#[cfg(feature = "msgpack")]
impl From<rmp_serde::encode::Error> for CerealError {
    fn from(error: rmp_serde::encode::Error) -> Self {
        CerealError::MsgPackSaveLoadFailed {
            error: error.to_string(),
        }
    }
}

#[cfg(feature = "csv")]
impl From<csv::Error> for CerealError {
    fn from(error: csv::Error) -> Self {
        CerealError::CsvSaveLoadFailed {
            error: error.to_string(),
        }
    }
}

#[cfg(feature = "yaml")]
impl From<serde_yaml::Error> for CerealError {
    fn from(error: serde_yaml::Error) -> Self {
        CerealError::YamlSaveLoadFailed {
            error: error.to_string(),
        }
    }
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for CerealError {
    fn from(error: serde_json::Error) -> Self {
        CerealError::JsonSaveLoadFailed {
            error: error.to_string(),
        }
    }
}
