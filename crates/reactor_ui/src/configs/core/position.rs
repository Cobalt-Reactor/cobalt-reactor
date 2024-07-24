use bevy::prelude::*;

/// Configuration anything with a position.
#[derive(Default, Debug)]
pub struct ReactorPosition {
    /// The type of position to use. Absolute or relative. Default is relative.
    pub position_type: PositionType,
    /// The distance from the left edge of the parent container.
    pub left: Option<Val>,
    /// The distance from the right edge of the parent container.
    pub right: Option<Val>,
    /// The distance from the top edge of the parent container.
    pub top: Option<Val>,
    /// The distance from the bottom edge of the parent container.
    pub bottom: Option<Val>,
}
