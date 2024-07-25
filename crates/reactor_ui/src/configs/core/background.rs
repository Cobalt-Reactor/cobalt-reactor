use crate::prelude::*;
use bevy::prelude::*;

/// Configuration for widget backgrounds.
#[derive(Debug, Clone)]
pub enum ReactorBackground {
    /// A background using an image source.
    Image(ReactorImageBackground),
    /// A flat background.
    Flat(ReactorFlatBackground),
}

impl Default for ReactorBackground {
    fn default() -> Self {
        Self::Flat(ReactorFlatBackground::default())
    }
}

/// Config for flat backgrounds
#[derive(Default, Debug, Clone)]
pub struct ReactorFlatBackground {
    /// The color of the background.
    pub background_color: Color,
    /// The color of the border.
    pub border_color: Color,
    /// The width of the border.
    pub corner_radius: ReactorCornerRadius,
    /// The thickness of the border.
    pub border_width: Option<Val>,
}
