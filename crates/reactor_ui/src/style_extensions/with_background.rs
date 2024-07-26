use crate::{prelude::*, sickle::prelude::*};
use bevy::prelude::*;

/// Fast handling of `ReactorBackground`.
pub trait UiStyleWithBackgroundExt {
    /// Calls `callback` when the widget is clicked.
    fn with_background(&mut self, background: &ReactorBackground) -> &mut Self;
}

impl UiStyleWithBackgroundExt for UiBuilder<'_, Entity> {
    fn with_background(&mut self, background: &ReactorBackground) -> &mut Self {
        let background = background.clone();

        match background {
            ReactorBackground::Image(image) => {
                self.entity_commands().insert(ImageBundle::default());
                self.style().image(image.into());
            }
            ReactorBackground::Flat(flat) => {
                self.entity_commands().insert(NodeBundle::default());

                if let Some(color) = flat.background_color {
                    self.style().background_color(color);
                }

                if let Some(corner_radius) = flat.corner_radius {
                    self.style().border_radius(corner_radius.into());
                }

                if let Some(config) = flat.border_config {
                    self.style()
                        .border_color(config.border_color)
                        .border(config.border_width);
                }
            }
            ReactorBackground::None => {}
        };

        self
    }
}
