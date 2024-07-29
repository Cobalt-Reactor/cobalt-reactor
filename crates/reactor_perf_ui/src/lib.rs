#![doc = include_str!("../README.md")]
#![expect(unused_imports)]
mod app_ext;
mod entries;
mod panels;
mod plugin;
mod schedule;
mod utils;
mod widgets;

/// Crate prelude
pub mod prelude {
    pub(crate) use crate::{app_ext::*, panels::*, widgets::*};
    pub use crate::{panels::*, plugin::ReactorPerfUiPlugin, schedule::ReactorPerfUiSchedule};
}
