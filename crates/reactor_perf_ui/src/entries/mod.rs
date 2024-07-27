mod entry_trait;
pub use entry_trait::*;
mod fps;
pub use fps::*;
mod window;
pub use window::*;
mod ecs;
pub use ecs::*;
#[cfg(feature = "sysinfo")]
mod system;
#[cfg(feature = "sysinfo")]
pub use system::*;
mod time;
pub use time::*;

// use bevy::prelude::*;

// #[derive(Component, Debug, Clone)]
// pub struct PerfUiEntryWindowMode;

// #[derive(Component, Debug, Clone)]
// pub struct PerfUiEntryWindowPresentMode;

// #[derive(Component, Debug, Clone)]
// pub struct PerfUiEntryWindowScaleFactor;

// #[derive(Component, Debug, Clone)]
// pub struct PerfUiEntryWindowResolution;

// #[derive(Component, Debug, Clone)]
// pub struct PerfUiEntryCursorPosition;

// #[cfg(feature = "sysinfo")]
// #[derive(Component, Debug, Clone)]
// pub struct PerfUiEntryMemUsage;
