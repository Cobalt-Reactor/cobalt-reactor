use crate::prelude::*;

/// Configuration for a button widget.
#[derive(Default, Debug, Clone)]
pub struct ReactorTextLabelConfig {
    /// The base config of the button (size, position, alignment, etc).
    pub base_config: ReactorBaseConfig,
    /// The text label to display on the button.
    pub label: ReactorLabelConfig,
    /// The font config for the button's label.
    pub font: Option<ReactorFontConfig>,
}

impl From<&str> for ReactorTextLabelConfig {
    fn from(text: &str) -> Self {
        Self {
            label: ReactorLabelConfig {
                text: text.to_string(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
