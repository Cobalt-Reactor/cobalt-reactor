#![allow(dead_code)]
use bevy::{asset::embedded_asset, prelude::*};

pub(crate) fn load(app: &mut App) {
    embedded_asset!(app, "bold.ttf");
    embedded_asset!(app, "narrow.ttf");
    embedded_asset!(app, "square.ttf");
    embedded_asset!(app, "std.ttf");
}

/// Default bold font
pub const BOLD: &str = "embedded://reactor_ui/fonts/bold.ttf";

/// Default narrow font
pub const NARROW: &str = "embedded://reactor_ui/fonts/narrow.ttf";

/// Default square font
pub const SQUARE: &str = "embedded://reactor_ui/fonts/square.ttf";

/// Default standard font
pub const STD: &str = "embedded://reactor_ui/fonts/std.ttf";
