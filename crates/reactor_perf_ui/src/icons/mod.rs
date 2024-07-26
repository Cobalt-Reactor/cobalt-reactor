#![allow(dead_code)]
use bevy::{self, asset::embedded_asset, prelude::*};

pub fn load(app: &mut App) {
    embedded_asset!(app, "expand.png");
    embedded_asset!(app, "collapse.png");
}

pub const EXPAND: &str = "embedded://reactor_perf_ui/icons/expand.png";
pub const COLLAPSE: &str = "embedded://reactor_perf_ui/icons/collapse.png";
