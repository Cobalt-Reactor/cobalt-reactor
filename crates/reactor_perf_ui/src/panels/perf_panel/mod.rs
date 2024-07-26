mod config;
mod ui_config;
mod widget_impl;
pub use config::*;

use bevy::prelude::*;
use reactor_ui::sickle::{theme::UiContext, UiContext};

#[derive(Debug, Component, Default, UiContext)]
pub struct ReactorPerfPanel;

impl ReactorPerfPanel {
    pub fn new() -> Self {
        Default::default()
    }
}
