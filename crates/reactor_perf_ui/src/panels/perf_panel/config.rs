use bevy::prelude::*;

/// Configuration for the performance panel.
#[derive(Resource, Debug, Clone)]
pub struct PerfPanelConfig {
    /// Show FPS data.
    pub(crate) fps: bool,
    /// Show window data.
    pub(crate) window: bool,
    /// Show ECS data.
    pub(crate) ecs: bool,
    /// Show system data.
    #[cfg(feature = "sysinfo")]
    pub(crate) system: bool,
    /// Show time data.
    pub(crate) time: bool,
}

impl PerfPanelConfig {
    /// Show FPS data.
    pub fn fps() -> Self {
        Self {
            fps: true,
            ..default()
        }
    }

    /// Show FPS and ECS data.
    pub fn minimal() -> Self {
        Self {
            fps: true,
            time: true,
            ..default()
        }
    }

    /// Show all available data.
    pub fn full() -> Self {
        Self {
            fps: true,
            window: true,
            ecs: true,
            #[cfg(feature = "sysinfo")]
            system: true,
            time: true,
        }
    }
}

impl Default for PerfPanelConfig {
    fn default() -> Self {
        Self::full()
    }
}
