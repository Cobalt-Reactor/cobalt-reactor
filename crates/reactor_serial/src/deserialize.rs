#[cfg(feature = "toml")]
use std::str::from_utf8;

use crate::prelude::*;

pub fn deserialize<T: SaveData>(input: &[u8], _format: SaveFormat) -> Result<T, CerealError> {
    match T::FORMAT {
        #[cfg(feature = "ron")]
        SaveFormat::Ron => {
            let out = ron::de::from_bytes(input)?;
            Ok(out)
        }
        #[cfg(feature = "json")]
        SaveFormat::Json => {
            let out = serde_json::from_slice(input)?;
            Ok(out)
        }
        #[cfg(feature = "yaml")]
        SaveFormat::Yaml => {
            let out = serde_yaml::from_slice(input)?;
            Ok(out)
        }
        #[cfg(feature = "toml")]
        SaveFormat::Toml => {
            let out = toml::from_str(from_utf8(input)?)?;
            Ok(out)
        }
        #[cfg(feature = "xml")]
        SaveFormat::Xml => {
            let out = quick_xml::de::from_reader(input)?;
            Ok(out)
        }
        #[cfg(feature = "csv")]
        SaveFormat::Csv => {
            let mut reader = csv::Reader::from_reader(input);
            let out = reader.deserialize().next().unwrap()?;
            Ok(out)
        }
        #[cfg(feature = "msgpack")]
        SaveFormat::MsgPack => {
            let out = rmp_serde::from_slice(input)?;
            Ok(out)
        }
    }
}
