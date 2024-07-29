mod config;
mod widget_impl;
pub use config::*;

use bevy::prelude::*;
use reactor_ui::sickle::{theme::UiContext, UiContext};

/// Marker trait for the Hierarchy panel
#[derive(Debug, Component, Default, UiContext)]
pub struct ReactorHierarchyPanel;
