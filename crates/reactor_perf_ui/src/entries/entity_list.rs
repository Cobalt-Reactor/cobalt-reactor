use super::{default_entry_header_config, PerfUiEntry};
use crate::{prelude::*, utils};
use bevy::{
    diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin},
    prelude::*,
};
use reactor_ui::{prelude::*, sickle::prelude::*};

#[derive(Component, Debug, Clone, Default)]
pub struct HierarchyUiEntryEntityList;

#[derive(Component, Debug, Clone, Default)]
pub struct HierarchyUiEntryEntityListContainer;

impl PerfUiEntry for HierarchyUiEntryEntityList {
    fn setup(_: &mut App) {}

    fn spawn(list: &mut UiBuilder<Entity>) {
        let config = default_entry_header_config("Entities".into());

        list.list_item_collapsible_header(config, |collapse| {
            collapse.insert(HierarchyUiEntryEntityListContainer);
        });

        list.insert(HierarchyUiEntryEntityList);
    }
}
