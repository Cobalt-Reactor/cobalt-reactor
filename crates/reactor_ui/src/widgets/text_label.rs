use crate::{prelude::*, sickle::prelude::*};
use bevy::prelude::*;

#[derive(Component)]
struct ReactorTextLabel;

/// Fast way to create a button
pub trait UiReactorTextLabelExt<'w, 's> {
    /// Creates a button.
    fn text_label(&mut self, config: ReactorTextLabelConfig) -> UiBuilder<Entity>;
}

impl<'w, 's> UiReactorTextLabelExt<'w, 's> for UiBuilder<'_, Entity> {
    /// Creates a button.
    /// Returns an `UiBuilder` for further customization.
    fn text_label(&mut self, config: ReactorTextLabelConfig) -> UiBuilder<Entity> {
        let mut label = self.label(config.label.into());

        label
            .style()
            .with_alignment(&config.base_config.alignment)
            .with_size(&config.base_config.size)
            .with_position(&config.base_config.position)
            .width(Val::Percent(100.0))
            .min_width(Val::Percent(100.0))
            .entity_commands()
            .insert(ReactorTextLabel);

        if let Some(font) = config.font {
            label.entity_commands().with_font(font);
        }

        label
    }
}

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
