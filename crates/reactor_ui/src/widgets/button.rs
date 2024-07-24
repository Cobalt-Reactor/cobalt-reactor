use crate::sickle::{prelude::*, ui_commands::*};
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
        self.container((ImageBundle::default(), ReactorButton), |banner| {
            banner
                .style()
                .position_type(PositionType::Absolute)
                // Center the children (the label) horizontally.
                .justify_content(JustifyContent::Center)
                .width(Val::Px(401.0))
                .height(Val::Px(79.0));

            if let Some(image) = config.image {
                banner.style().image(image);
            }

            if let Some(text) = config.label {
                // And we'll want a customizable label on the banner.
                let mut label = banner.label(LabelConfig::default());

                label
                    .style()
                    // Align the label relative to the top of the banner.
                    .align_self(AlignSelf::Start)
                    // Move us a few pixels down so we look nice relative to our font.
                    .top(Val::Px(20.0));

                // We would like to set a default text style without having to pass in the
                // AssetServer.
                label.entity_commands().set_text(text, None);
            }
        })
    }
}

pub struct ReactorButtonConfig {
    pub label: Option<String>,
    pub image: Option<ImageSource>,
}
