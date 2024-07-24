use bevy::prelude::TextStyle;

use crate::prelude::*;

/// Configuration for text to be added to a widget
#[derive(Default, Debug)]
pub struct ReactorText {
    /// The text to display
    pub text: String,
    /// The style of the text
    pub style: Option<TextStyle>,
    /// The font config for the text
    pub font_config: Option<ReactorFontConfig>,
}
