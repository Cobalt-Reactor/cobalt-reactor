use crate::prelude::*;
use bevy::prelude::*;

/// A 2D spatial bundle that does not include the bevy
/// `SpatialBundle`
#[derive(Bundle, Default)]
pub struct SpatialBundle2DRaw {
    /// Position in 2D space
    pub position: Position2D,
    /// Rotation in 2D space
    pub rotation: Rotation2D,
    /// Scale in 2D space
    pub scale: Scale2D,
    /// Draw order
    pub draw_order: DrawOrder,
    /// Rotation propagation mode
    pub r_prop: RotationPropagation,
    /// Position propagation mode
    pub p_prop: PositionPropagation,
    /// Scale propagation mode
    pub s_prop: ScalePropagation,
}

/// A 2D spatial bundle that includes the bevy `SpatialBundle`
#[derive(Bundle, Default)]
pub struct SpatialBundle2D {
    /// Position in 2D space
    pub position: Position2D,
    /// Rotation in 2D space
    pub rotation: Rotation2D,
    /// Scale in 2D space
    pub scale: Scale2D,
    /// Draw order
    pub draw_order: DrawOrder,
    /// Rotation propagation mode
    pub r_prop: RotationPropagation,
    /// Position propagation mode
    pub p_prop: PositionPropagation,
    /// Scale propagation mode
    pub s_prop: ScalePropagation,
    /// The bevy `SpatialBundle` (required for `GlobalTransform`, `Transform`, 'Visibility', etc)
    pub spatial: SpatialBundle,
}
