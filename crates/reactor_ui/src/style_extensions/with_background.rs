use crate::{prelude::*, sickle::prelude::*};
use bevy::prelude::*;

/// Fast handling of `ReactorBackground`.
pub trait StyleWithBackgroundExt {
    /// Calls `callback` when the widget is clicked.
    fn with_background(&mut self, background: &ReactorBackground) -> &mut Self;
}

impl StyleWithBackgroundExt for UiBuilder<'_, Entity> {
    fn with_background(&mut self, background: &ReactorBackground) -> &mut Self {
        let background = background.clone();

        match background {
            ReactorBackground::Image(image) => {
                self.entity_commands().insert(ImageBundle::default());
                self.style().image(image.into());
            }
            ReactorBackground::Flat(flat) => {
                self.entity_commands().insert(NodeBundle::default());
                self.style()
                    .border_color(flat.border_color)
                    .background_color(flat.background_color)
                    .border_radius(flat.corner_radius.into());

                if let Some(border_width) = flat.border_width {
                    self.style().border(UiRect::all(border_width));
                }
            }
        };

        self
    }
}
