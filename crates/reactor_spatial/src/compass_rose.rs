use bevy::prelude::*;

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Hash, Component, Reflect)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A 8 direction compass component
pub enum CompassRose {
    #[default]
    /// North
    N,
    /// North-east
    NE,
    /// East
    E,
    /// South-east
    SE,
    /// South
    S,
    /// South-west
    SW,
    /// West
    W,
    /// North-west
    NW,
}

mod from {
    use super::CompassRose;
    use crate::{
        compass::Compass,
        components::Rotation2D,
        math::{Degrees, Radians},
    };

    impl From<Degrees> for CompassRose {
        fn from(degrees: Degrees) -> Self {
            let angle = degrees.to_f32();
            let mut normalized_angle = ((angle % 360.0) + 360.) % 360.0;
            if normalized_angle > 180. {
                normalized_angle -= 360.;
            }
            match normalized_angle {
                -112.5..=-67.5 => Self::S,
                -67.5..=-22.5 => Self::SE,
                -22.5..=22.5 => Self::E,
                22.5..=67.5 => Self::NE,
                67.5..=112.5 => Self::N,
                112.5..=157.5 => Self::NW,
                157.5..=180.0 | -180.0..=-157.5 => Self::W,
                -157.5..=-112.5 => Self::SW,
                _ => unreachable!(),
            }
        }
    }

    impl From<&Degrees> for CompassRose {
        fn from(degrees: &Degrees) -> Self {
            Self::from(*degrees)
        }
    }

    impl From<Radians> for CompassRose {
        fn from(radians: Radians) -> Self {
            Self::from(Degrees::from(radians))
        }
    }

    impl From<&Radians> for CompassRose {
        fn from(radians: &Radians) -> Self {
            Self::from(*radians)
        }
    }

    impl From<Rotation2D> for CompassRose {
        fn from(rotation: Rotation2D) -> Self {
            Self::from(Degrees::from(rotation))
        }
    }

    impl From<&Rotation2D> for CompassRose {
        fn from(rotation: &Rotation2D) -> Self {
            Self::from(*rotation)
        }
    }

    impl From<Compass> for CompassRose {
        fn from(compass: Compass) -> Self {
            match compass {
                Compass::N => Self::N,
                Compass::S => Self::S,
                Compass::E => Self::E,
                Compass::W => Self::W,
            }
        }
    }

    impl From<&Compass> for CompassRose {
        fn from(compass: &Compass) -> Self {
            Self::from(*compass)
        }
    }
}

mod into {
    use super::CompassRose;
    use crate::math::Radians;
    use bevy::prelude::*;

    impl From<CompassRose> for Vec2 {
        fn from(compass: CompassRose) -> Self {
            Vec2::from(Radians::from(compass))
        }
    }

    impl From<&CompassRose> for Vec2 {
        fn from(compass: &CompassRose) -> Self {
            (*compass).into()
        }
    }
}
