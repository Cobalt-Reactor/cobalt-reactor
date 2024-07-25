use crate::{prelude::*, sickle::prelude::*};
use bevy::prelude::*;

#[derive(Component)]
struct ReactorButton;

/// Fast way to create a button
pub trait UiReactorButtonExt<'w, 's> {
    /// Creates a button.
    fn button(&mut self, config: ReactorButtonConfig) -> UiBuilder<Entity>;
}

impl<'w, 's> UiReactorButtonExt<'w, 's> for UiBuilder<'_, Entity> {
    /// Creates a button.
    /// Returns an `UiBuilder` for further customization.
    fn button(&mut self, config: ReactorButtonConfig) -> UiBuilder<Entity> {
        self.container(ReactorButton, |button| {
            button
                .with_background(&config.background)
                .with_base_config(&config.base_config);

            if let Some(label_config) = config.label {
                button.text_label(label_config);
            }
        })
    }
}

/// Configuration for a button widget.
#[derive(Default, Debug, Clone)]
pub struct ReactorButtonConfig {
    /// The base config of the button (size, position, alignment, etc).
    pub base_config: ReactorBaseConfig,
    /// The image to display as the button's background.
    pub background: ReactorBackground,
    /// The text label to display on the button.
    pub label: Option<ReactorTextLabelConfig>,
}
