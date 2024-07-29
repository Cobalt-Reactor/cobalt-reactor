#![allow(dead_code)]
use bevy::{self, asset::embedded_asset, prelude::*};

pub(crate) fn load(app: &mut App) {
    embedded_asset!(app, "expand.png");
    embedded_asset!(app, "collapse.png");
}

/// Default expand icon
pub const EXPAND: &str = "embedded://reactor_ui/icons/expand.png";

/// Default collapse icon
pub const COLLAPSE: &str = "embedded://reactor_ui/icons/collapse.png";
