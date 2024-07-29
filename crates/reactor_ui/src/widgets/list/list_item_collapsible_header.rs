use crate::{prelude::*, sickle::prelude::*};
use bevy::prelude::*;

#[derive(Component)]
struct ListItemCollapsibleHeader;

/// Fast way to create a list item
pub trait UiListItemCollapsibleHeaderExt<'w, 's> {
    /// Creates a list item.
    fn list_item_collapsible_header(
        &mut self,
        config: ReactorCollapsibleConfig,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity>;
}

impl<'w, 's> UiListItemCollapsibleHeaderExt<'w, 's> for UiBuilder<'_, Entity> {
    /// Creates a list item
    /// Returns an `UiBuilder` for further customization.
    fn list_item_collapsible_header(
        &mut self,
        config: ReactorCollapsibleConfig,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity> {
        let mut collapsible = self.collapsible(config, |collapse| {
            spawn_children(collapse);
        });

        collapsible
            .style()
            .justify_items(JustifyItems::Start)
            .align_items(AlignItems::Center)
            .entity_commands()
            .insert(ListItemCollapsibleHeader);

        collapsible
    }
}
