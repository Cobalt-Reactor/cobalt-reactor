use crate::prelude::*;
use bevy::prelude::*;

/// Configuration for widget backgrounds.
#[derive(Debug, Clone)]
pub enum ReactorBackground {
    /// A background using an image source.
    Image(ReactorImageBackground),
    /// A flat background.
    Flat(ReactorFlatBackground),
    /// No background.
    None,
}

impl Default for ReactorBackground {
    fn default() -> Self {
        Self::None
    }
}

/// Config for flat backgrounds
#[derive(Default, Debug, Clone)]
pub struct ReactorFlatBackground {
    /// The color of the background.
    pub background_color: Option<Color>,
    /// The width of the border.
    pub corner_radius: Option<ReactorCornerRadius>,
    /// The border of the background.
    pub border_config: Option<ReactorBorder>,
}
