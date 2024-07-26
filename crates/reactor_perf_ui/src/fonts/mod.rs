#![allow(dead_code)]
use bevy::{asset::embedded_asset, prelude::*};

pub fn load(app: &mut App) {
    embedded_asset!(app, "bold.ttf");
    embedded_asset!(app, "narrow.ttf");
    embedded_asset!(app, "square.ttf");
    embedded_asset!(app, "std.ttf");
}

pub const BOLD: &str = "embedded://reactor_perf_ui/fonts/bold.ttf";
pub const NARROW: &str = "embedded://reactor_perf_ui/fonts/narrow.ttf";
pub const SQUARE: &str = "embedded://reactor_perf_ui/fonts/square.ttf";
pub const STD: &str = "embedded://reactor_perf_ui/fonts/std.ttf";
