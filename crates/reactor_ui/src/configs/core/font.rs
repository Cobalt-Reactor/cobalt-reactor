use bevy::prelude::*;

/// Configuration anything with a size.
#[derive(Default, Debug)]
pub struct ReactorFontConfig {
    /// The path to the font file.
    pub path: String,
    /// The size of the font.
    pub size: f32,
    /// The color of the font.
    pub color: Color,
}
