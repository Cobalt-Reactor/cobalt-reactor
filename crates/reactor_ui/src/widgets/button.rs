use crate::{prelude::*, sickle::prelude::*};
use bevy::prelude::*;

#[derive(Component)]
struct ReactorButton;

/// Fast way to create a button
pub trait UiReactorButtonExt<'w, 's> {
    /// Creates a button.
    fn button(&mut self, config: ReactorButtonConfig) -> UiBuilder<Entity>;
}

impl<'w, 's> UiReactorButtonExt<'w, 's> for UiBuilder<'_, UiRoot> {
    /// Creates a button.
    /// Returns an `UiBuilder` for further customization.
    fn button(&mut self, config: ReactorButtonConfig) -> UiBuilder<Entity> {
        self.container((ImageBundle::default(), ReactorButton), |button| {
            button
                .style()
                .with_alignment(&config.base_config.alignment)
                .with_size(&config.base_config.size)
                .with_position(&config.base_config.position);

            if let Some(image) = config.image {
                button.style().image(image);
            }

            if let Some(label_config) = config.label {
                button.text_label(label_config);
            }

            if let Some(picking) = config.base_config.picking {
                button
                    .entity_commands()
                    .pickable(picking.block_lower, picking.hoverable);
            }
        })
    }
}
