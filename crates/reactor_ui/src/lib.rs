#![doc = include_str!("../README.md")]
mod extensions;
mod plugin;
mod widgets;

/// The contents of this module are re-exported by the `reactor_ui` crate.
pub mod prelude {
    pub use crate::{extensions::*, plugin::UiPlugin, widgets::*};
}

/// Re-export of `sickle_ui` for convenience.
pub mod sickle {
    pub use sickle_ui::*;
}

/// Re-export of `bevy_mod_picking` for convenience.
pub mod picking {
    pub use bevy_mod_picking::prelude::*;
}
