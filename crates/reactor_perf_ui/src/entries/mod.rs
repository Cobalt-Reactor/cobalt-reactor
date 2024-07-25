mod entry_trait;
pub use entry_trait::*;
mod fps;
pub use fps::*;
mod window;
pub use window::*;
mod ecs;
pub use ecs::*;
mod system;
pub use system::*;

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

// #[derive(Component, Debug, Clone)]
// pub struct PerfUiEntryRunningTime;

// #[derive(Component, Debug, Clone)]
// pub struct PerfUiEntryClock;

// #[derive(Component, Debug, Clone)]
// pub struct PerfUiEntryFixedTimeStepDuration;

// #[derive(Component, Debug, Clone)]
// pub struct PerfUiEntryFixedOverstep;

// #[derive(Component, Debug, Clone)]
// pub struct PerfUiEntryFPS;

// #[derive(Component, Debug, Clone)]
// pub struct PerfUiEntryFPSWorst;

// #[derive(Component, Debug, Clone)]
// pub struct PerfUiEntryFrameTime;

// #[derive(Component, Debug, Clone)]
// pub struct PerfUiEntryFrameCount;

// #[derive(Component, Debug, Clone)]
// pub struct PerfUiEntryEntityCount;

// #[cfg(feature = "sysinfo")]
// #[derive(Component, Debug, Clone)]
// pub struct PerfUiEntryCpuUsage;

// #[cfg(feature = "sysinfo")]
// #[derive(Component, Debug, Clone)]
// pub struct PerfUiEntryMemUsage;
