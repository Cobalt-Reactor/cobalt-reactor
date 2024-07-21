use crate::prelude::*;

pub fn serialize<T: SaveData>(input: &T) -> Result<Vec<u8>, CerealError> {
    match T::FORMAT {
        #[cfg(feature = "ron")]
        SaveFormat::Ron => {
            let out = ron::to_string(input)?;
            Ok(out.into_bytes())
        }
        #[cfg(feature = "json")]
        SaveFormat::Json => {
            let out = serde_json::to_string(input)?;
            Ok(out.into_bytes())
        }
        #[cfg(feature = "yaml")]
        SaveFormat::Yaml => {
            let out = serde_yaml::to_string(input)?;
            Ok(out.into_bytes())
        }
        #[cfg(feature = "toml")]
        SaveFormat::Toml => {
            let out = toml::to_string(input)?;
            Ok(out.into_bytes())
        }
        #[cfg(feature = "xml")]
        SaveFormat::Xml => {
            let out = quick_xml::se::to_string(input)?;
            Ok(out.into_bytes())
        }
        #[cfg(feature = "csv")]
        SaveFormat::Csv => {
            let mut builder = csv::WriterBuilder::new().from_writer(Vec::new());
            builder.serialize(input)?;
            // Should never fail because we're writing to a vec not a file
            let out = builder.into_inner().unwrap();
            Ok(out)
        }
        #[cfg(feature = "msgpack")]
        SaveFormat::MsgPack => {
            let out = rmp_serde::to_vec(input)?;
            Ok(out)
        }
    }
}
