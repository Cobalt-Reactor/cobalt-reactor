use std::any::type_name;

use crate::{error::CerealError, prelude::*};

/// Converts one save format to another as long as they are associated types
pub fn convert_save<T: SaveData>(
    save_version: SaveVersion,
    file_name: &str,
    data: &[u8],
) -> Result<T, CerealError> {
    if T::VERSION == TerminalSaveData::VERSION {
        return Err(CerealError::NoOptionInChain {
            file_name: file_name.to_string(),
            to: type_name::<T>().to_string(),
        });
    }

    if T::VERSION == T::Previous::VERSION {
        return Err(CerealError::VersionChainDuplicate {
            file_name: file_name.to_string(),
        });
    }

    if T::VERSION < T::Previous::VERSION {
        return Err(CerealError::VersionChainDecrement {
            file_name: file_name.to_string(),
        });
    }

    if save_version == T::VERSION {
        return deserialize::<T>(data, T::FORMAT);
    }

    let recur_convert = convert_save::<T::Previous>(save_version, file_name, data)?;
    let output = T::from_previous(&recur_convert);
    match output {
        Some(output) => Ok(output),
        None => Err(CerealError::VersionUpgradeFailed {
            file_name: file_name.to_string(),
            from: type_name::<T::Previous>().to_string(),
            to: type_name::<T>().to_string(),
        }),
    }
}
