use super::{default_collapsible_content_config, default_collapsible_header_config, PerfUiEntry};
use crate::{prelude::*, utils};
use bevy::{
    diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin},
    prelude::*,
    utils::info,
};
use reactor_ui::{prelude::*, sickle::prelude::*};

#[derive(Component, Debug, Clone, Default)]
pub struct HierarchyUiEntryEntityList;

#[derive(Component, Debug, Clone, Default)]
pub struct HierarchyUiEntryEntityListContainer;

impl PerfUiEntry for HierarchyUiEntryEntityList {
    fn setup(app: &mut App) {
        app.add_systems(Update, add_entities.in_set(ReactorPerfUiSchedule::Update));
    }

    fn spawn(list: &mut UiBuilder<Entity>) {
        list.list_item_collapsible_header(
            default_collapsible_header_config("Entities".into()),
            |container| {
                container.insert(HierarchyUiEntryEntityListContainer);
            },
        );

        list.insert(HierarchyUiEntryEntityList);
    }
}

fn add_entities(
    mut commands: Commands,
    root_entities: Query<(Entity, Option<&Name>), Without<Parent>>,
    containers: Query<Entity, With<HierarchyUiEntryEntityListContainer>>,
    mut run_once: Local<bool>,
) {
    if *run_once {
        return;
    }

    for container in containers.iter() {
        *run_once = true;

        info!("Adding initial entities");
        let mut builder = commands.ui_builder(container);

        for (entity, name) in root_entities.iter() {
            info!("Adding entity {}", entity);
            let generation = entity.generation();
            let index = entity.index();
            let name = match name {
                Some(name) => name.to_string(),
                None => "Entity".to_string(),
            };
            let full_name = format!("{} {}v{}", name, index, generation);

            builder.list_item_collapsible_content(
                default_collapsible_content_config(full_name),
                |_| {},
            );
        }
    }
}
