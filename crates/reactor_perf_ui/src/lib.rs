#![doc = include_str!("../README.md")]
mod app_ext;
mod defines;
mod entries;
mod plugin;
mod schedule;
mod utils;
mod widgets;

/// Crate prelude
pub mod prelude {
    pub(crate) use crate::{app_ext::*, defines::*, widgets::*};
    pub use crate::{
        plugin::ReactorPerfUiPlugin, schedule::ReactorPerfUiSchedule, widgets::PerfPanelConfig,
    };
}
