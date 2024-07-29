use bevy::prelude::*;

/// Configuration anything with a position.
#[derive(Debug, Clone)]
pub enum ReactorPosition {
    /// Absolute position in screen space in pixels
    Absolute(ReactorPositionAbsolute),
    /// Relative position as Val compared to parent
    Relative(ReactorPositionRelative),
}

impl Default for ReactorPosition {
    fn default() -> Self {
        ReactorPosition::Relative(ReactorPositionRelative::default())
    }
}

/// Absolute position in screen space in pixels
#[derive(Default, Debug, Clone)]
pub struct ReactorPositionAbsolute {
    /// X position
    pub x: f32,
    /// Y position
    pub y: f32,
}

/// Relative position as Val compared to parent
#[derive(Default, Debug, Clone)]
pub struct ReactorPositionRelative {
    /// Top position
    pub top: Val,
    /// Bottom position
    pub bottom: Val,
    /// Left position
    pub left: Val,
    /// Right position
    pub right: Val,
}
