use crate::{entries::entity_entry_config, prelude::*};
use bevy::prelude::*;
use reactor_ui::{prelude::*, sickle::prelude::*};

#[derive(Component)]
struct HierarchyEntityEntry;

#[derive(Component)]
struct HierarchyEntityEntryHeader;

#[derive(Component)]
struct HierarchyEntityEntryContent;

/// Fast way to create a list item
pub trait UiHierarchyEntityEntryExt<'w, 's> {
    /// Creates a list item.
    fn entity_entry(&mut self, entity: Entity, name: String) -> (Entity, Entity);
}

impl<'w, 's> UiHierarchyEntityEntryExt<'w, 's> for UiBuilder<'_, Entity> {
    /// Creates a list item
    /// Returns an `UiBuilder` for further customization.
    fn entity_entry(&mut self, _entity: Entity, name: String) -> (Entity, Entity) {
        let config = entity_entry_config(name);
        let mut content_container_id = None;

        let mut container = self.column(|entity_root| {
            entity_root
                .list_item_collapsible_header(config, |content_container| {
                    content_container
                        .entity_commands()
                        .insert(HierarchyEntityEntryContent);

                    content_container_id = Some(content_container.id());
                })
                .entity_commands()
                .insert(HierarchyEntityEntryHeader);
        });

        container
            .style()
            .width(Val::Percent(100.0))
            .padding(UiRect::left(Val::Percent(5.0)))
            .entity_commands()
            .insert(HierarchyEntityEntry)
            .insert(Name::new("Entity Entry"));

        (container.id(), content_container_id.unwrap())
    }
}
