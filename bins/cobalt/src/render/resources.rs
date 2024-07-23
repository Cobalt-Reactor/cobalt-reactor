use bevy::prelude::*;
use reactor_core::spatial::Position2D;

#[derive(Resource, Default)]
pub struct MouseWorldCoords(pub Position2D);
