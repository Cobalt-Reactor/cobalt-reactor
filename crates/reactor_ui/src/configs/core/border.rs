use bevy::prelude::{Color, UiRect};

/// Config for flat backgrounds
#[derive(Default, Debug, Clone)]
pub struct ReactorBorder {
    /// The si
    /// The color of the border.
    pub border_color: Color,
    /// The thickness of the border.
    pub border_width: UiRect,
}
