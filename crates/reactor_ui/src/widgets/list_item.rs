use crate::{prelude::*, sickle::prelude::*};
use bevy::prelude::*;

#[derive(Component)]
struct ReactorListItem;

/// Fast way to create a list item
pub trait UiReactorListItemExt<'w, 's> {
    /// Creates a list item.
    fn list_item(
        &mut self,
        config: ReactorListItemConfig,
        content: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity>;
}

impl<'w, 's> UiReactorListItemExt<'w, 's> for UiBuilder<'_, Entity> {
    /// Creates a list item
    /// Returns an `UiBuilder` for further customization.
    fn list_item(
        &mut self,
        config: ReactorListItemConfig,
        content_builder: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity> {
        self.row(|row| {
            row.with_background(&config.background);
            row.style()
                .height(config.size.height)
                .justify_self(JustifySelf::Start)
                .align_self(AlignSelf::Start)
                .justify_items(JustifyItems::Start)
                .align_items(AlignItems::Start)
                .justify_content(JustifyContent::SpaceBetween)
                .entity_commands()
                .insert(ReactorListItem);

            content_builder(row);
        })
    }
}

/// Configuration for a list item widget.
#[derive(Default, Debug, Clone)]
pub struct ReactorListItemConfig {
    /// The background of the list item.
    pub background: ReactorBackground,
    /// The size of the list item
    pub size: ReactorSize,
}
