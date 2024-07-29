#![doc = include_str!("../README.md")]
#![feature(float_minimum_maximum)]
mod configs;
/// A number of built-in font options
pub mod fonts;
/// A number of built-in icon options
pub mod icons;
mod picking_ext;
mod plugin;
mod schedule;
mod style_ext;
mod widgets;

/// The contents of this module are re-exported by the `reactor_ui` crate.
pub mod prelude {
    pub use crate::{
        configs::*, fonts, icons, picking_ext::*, plugin::ReactorUiPlugin, schedule::*,
        style_ext::*, widgets::*,
    };
    pub use bevy::{
        color::{palettes::*, prelude::*},
        ui::prelude::*,
    };
}

/// Re-export of `sickle_ui` for convenience.
pub mod sickle {
    pub use sickle_ui::*;
}

/// Re-export of `bevy_mod_picking` for convenience.
pub mod picking {
    pub use bevy_mod_picking::prelude::*;
}
