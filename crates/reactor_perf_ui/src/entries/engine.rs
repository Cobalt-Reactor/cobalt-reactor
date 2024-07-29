use super::{default_collapsible_header_config, PerfUiEntry};
use crate::{prelude::*, utils};
use bevy::{
    diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin},
    prelude::*,
};
use reactor_ui::{prelude::*, sickle::prelude::*};

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryEcs;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryEcsEntityCountLabel;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryEcsEntityCountData;

impl PerfUiEntry for PerfUiEntryEcs {
    fn setup(app: &mut App) {
        app.add_plugins(EntityCountDiagnosticsPlugin);
        app.add_systems(
            Update,
            (update_entity_count).in_set(ReactorPerfUiSchedule::Update),
        );
    }
    fn spawn(list: &mut UiBuilder<Entity>) {
        let config = default_collapsible_header_config("Engine".into());

        list.list_item_collapsible_header(config, |collapse| {
            collapse.list_item_two_text(ListItemTwoTextConfig {
                title_text: "Ent Count:".to_string(),
                title_component: PerfUiEntryEcsEntityCountLabel,
                content_text: "1.0".to_string(),
                content_component: PerfUiEntryEcsEntityCountData,
            });
        });

        list.insert(PerfUiEntryEcs);
    }
}

fn update_entity_count(
    mut text_children: Query<&mut Text, With<PerfUiEntryEcsEntityCountData>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    for mut text in text_children.iter_mut() {
        let count = diagnostics
            .get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
            .and_then(bevy::diagnostic::Diagnostic::value)
            .unwrap_or(0.0) as i64;
        text.sections[0].value = utils::format_pretty_int(3, count);
    }
}
