use super::PerfUiEntry;
use crate::{prelude::*, utils};
use bevy::{
    diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin},
    prelude::*,
};
use reactor_ui::sickle::prelude::*;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryEngine;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryEngineEntityCountLabel;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryEngineEntityCountData;

impl PerfUiEntry for PerfUiEntryEngine {
    fn setup(app: &mut App) {
        app.add_plugins(EntityCountDiagnosticsPlugin);
        app.add_systems(
            Update,
            (Self::update_entity_count).in_set(ReactorPerfUiSchedule::Update),
        );
    }
    fn spawn(list: &mut UiBuilder<Entity>) {
        let config = PanelEntryCollapsibleConfig {
            label: "Engine".into(),
        };
        // Add a collapsible List Item here with other list items in it
        list.panel_entry_collapsible(config, |collapse| {
            collapse.panel_entry_text(PanelEntryTextConfig {
                title_text: "Ent Count:".to_string(),
                title_component: PerfUiEntryEngineEntityCountLabel,
                content_text: "1.0".to_string(),
                content_component: PerfUiEntryEngineEntityCountData,
            });
        });

        list.insert(PerfUiEntryEngine);
    }
}

impl PerfUiEntryEngine {
    fn update_entity_count(
        mut text_children: Query<&mut Text, With<PerfUiEntryEngineEntityCountData>>,
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
}
