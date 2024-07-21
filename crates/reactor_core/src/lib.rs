//! A container for all of my gamedev crates. See the README.md in `crates/reactor_core` for
//! details.
/// The main `reactor_core` plugin. Add this to your app.
#[cfg(feature = "plugin")]
mod plugin;
#[cfg(feature = "plugin")]
pub use plugin::ReactorCorePlugin;

/// `reactor_spatial`
#[cfg(feature = "spatial")]
pub mod spatial {
    pub use reactor_spatial::prelude::*;
}

/// `reactor_random`
#[cfg(feature = "random")]
pub mod random {
    pub use reactor_random::{prelude::*, seed};
}

/// `reactor_camera`
#[cfg(feature = "camera")]
pub mod camera {
    pub use reactor_camera::prelude::*;
}

/// `reactor_proto`
#[cfg(feature = "proto")]
pub mod proto {
    pub use reactor_proto::prelude::*;
}

/// `reactor_serial`
#[cfg(feature = "serial")]
pub mod serial {
    pub use reactor_serial::prelude::*;
}
