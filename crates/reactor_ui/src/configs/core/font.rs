use bevy::prelude::*;

/// Configuration anything with a size.
#[derive(Default, Debug, Clone)]
pub struct ReactorFontConfig {
    /// The path to the font file.
    pub font: ReactorFontType,
    /// The size of the font.
    pub size: f32,
    /// The color of the font.
    pub color: Color,
}

/// Configuration anything with a size.
#[derive(Default, Debug, Clone)]
pub enum ReactorFontType {
    /// The path to the font file.
    Path(String),
    /// A handle to the font
    Handle(Handle<Font>),
    /// Use the Bevy default font
    #[default]
    BuiltIn,
}

impl From<Handle<Font>> for ReactorFontType {
    fn from(handle: Handle<Font>) -> Self {
        Self::Handle(handle)
    }
}

impl From<String> for ReactorFontType {
    fn from(path: String) -> Self {
        Self::Path(path)
    }
}

impl From<&str> for ReactorFontType {
    fn from(path: &str) -> Self {
        Self::Path(path.to_string())
    }
}
