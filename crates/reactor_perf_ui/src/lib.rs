#![doc = include_str!("../README.md")]
mod app_ext;
mod entries;
mod fonts;
mod icons;
mod panels;
mod plugin;
mod schedule;
mod utils;
mod widgets;

/// Crate prelude
pub mod prelude {
    pub(crate) use crate::{app_ext::*, panels::*, widgets::*};
    pub use crate::{
        panels::PerfPanelConfig, plugin::ReactorPerfUiPlugin, schedule::ReactorPerfUiSchedule,
    };
}
