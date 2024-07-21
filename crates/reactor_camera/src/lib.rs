#![doc = include_str!("../README.md")]
/// All components available in the crate
pub mod components;
mod main_camera;
/// All plugins available in the crate
pub mod plugins;
mod systems;

/// Re-exports as prelude
pub mod prelude {
    pub use crate::{components::*, plugins::*};
    pub(crate) use crate::{main_camera::*, systems::*};
}
