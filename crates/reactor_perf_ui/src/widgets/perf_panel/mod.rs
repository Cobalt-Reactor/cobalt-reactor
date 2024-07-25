mod config;
mod ui_config;
mod widget_impl;
pub use config::*;

use bevy::prelude::*;

#[derive(Debug, Component, Default)]
pub struct ReactorPerfPanel;

impl ReactorPerfPanel {
    pub fn new() -> Self {
        Default::default()
    }
}
