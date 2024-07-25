/// All defines related to window management
pub mod window_defines {
    /// Window title
    pub const TITLE: &str = "Cobalt";
    /// Default resolution
    pub const RESOLUTION: (f32, f32) = (1920., 1080.);
}

/// All defines related to logging
pub mod logging_defines {
    /// Default log level
    #[cfg(debug_assertions)]
    pub const DEFAULT_LEVEL: bevy::log::Level = bevy::log::Level::DEBUG;
    /// Default log filter
    #[cfg(debug_assertions)]
    pub const DEFAULT_FILTER: &str =
        "error,wgpu_hal=off,reactor_core=debug,reactor_ui=debug,cobalt=debug";

    /// Default log level
    #[cfg(not(debug_assertions))]
    pub const DEFAULT_LEVEL: bevy::log::Level = bevy::log::Level::ERROR;
    /// Default log filter
    #[cfg(not(debug_assertions))]
    pub const DEFAULT_FILTER: &str = "error";
}
