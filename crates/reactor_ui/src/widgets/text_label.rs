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
        let mut label = self.label(config.label);

        label
            .style()
            .with_alignment(&config.base_config.alignment)
            .with_size(&config.base_config.size)
            .with_position(&config.base_config.position)
            .entity_commands()
            .insert(ReactorTextLabel);

        if let Some(font) = config.font {
            label.entity_commands().with_font(font);
        }

        label
    }
}
