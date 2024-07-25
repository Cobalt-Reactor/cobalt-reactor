use crate::prelude::*;

/// Configuration for text to be added to a widget
#[derive(Default, Debug, Clone)]
pub struct ReactorText {
    /// The text to display
    pub text: String,
    /// The font config for the text
    pub font_config: Option<ReactorFontConfig>,
}
