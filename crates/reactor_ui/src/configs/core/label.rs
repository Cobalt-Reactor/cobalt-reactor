use crate::sickle::prelude::LabelConfig;
use bevy::prelude::*;

/// Configuration for a text label widget.
#[derive(Debug, Default, Clone)]
pub struct ReactorLabelConfig {
    /// The text to display on the label.
    pub text: String,
    /// The color of the text, is overrode by font settings.
    pub color: Color,
    /// The margin of the label.
    pub margin: UiRect,
    /// The wrap setting of the label. See `sickle_ui` for more information.
    pub wrap: FlexWrap,
    /// The flex grow setting of the label. See `sickle_ui` for more information.
    pub flex_grow: f32,
}

impl From<ReactorLabelConfig> for LabelConfig {
    fn from(config: ReactorLabelConfig) -> Self {
        Self {
            label: config.text,
            color: config.color,
            margin: config.margin,
            wrap: config.wrap,
            flex_grow: config.flex_grow,
        }
    }
}

impl From<&str> for ReactorLabelConfig {
    fn from(text: &str) -> Self {
        Self {
            text: text.to_string(),
            ..Default::default()
        }
    }
}

impl From<String> for ReactorLabelConfig {
    fn from(text: String) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }
}
