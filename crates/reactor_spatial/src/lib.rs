#![doc = include_str!("../README.md")]
mod compass;
mod compass_halfwinds;
mod compass_rose;
mod degrees;
mod draw_order;
mod position2d;
mod propagation_systems;
mod radians;
#[cfg(feature = "random")]
mod random;
mod rotation2d;
mod scale2d;
mod spatial_bundle;
mod spatial_plugin;

/// All components available in the crate
pub mod components {
    pub use crate::{
        compass::Compass,
        compass_halfwinds::CompassHalfwinds,
        compass_rose::CompassRose,
        draw_order::DrawOrder,
        position2d::{Position2D, PositionPropagation},
        rotation2d::{Rotation2D, RotationPropagation},
        scale2d::{Scale2D, ScalePropagation},
        spatial_bundle::{SpatialBundle2D, SpatialBundle2DRaw},
    };
}

/// All math types available in the crate
pub mod math {
    pub use crate::{degrees::Degrees, radians::Radians};
}

/// All plugins available in the crate
pub mod plugins {
    pub use crate::spatial_plugin::ReactorSpatialPlugin;
}

/// All systems available in the crate
pub(crate) mod systems {
    pub use crate::propagation_systems::{
        propagate_spatial2d, update_compass_from_rotation2d,
        update_compass_halfwinds_from_rotation2d, update_compass_rose_from_rotation2d,
    };
}

/// All schedules available in the crate
pub mod schedules {
    pub use crate::propagation_systems::SpatialSystems2D;
}

/// Re-exports as prelude
pub mod prelude {
    pub(crate) use crate::systems::*;
    pub use crate::{components::*, math::*, plugins::*, schedules::*};
}
