#![doc = include_str!("../README.md")]
mod plugin;

/// Crate prelude
pub mod prelude {
    pub use crate::plugin::ReactorPerfUiPlugin;
}
