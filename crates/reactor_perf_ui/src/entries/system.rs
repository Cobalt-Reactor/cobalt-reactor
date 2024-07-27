use super::PerfUiEntry;
use crate::{prelude::*, utils};
use bevy::{
    diagnostic::{DiagnosticsStore, SystemInformationDiagnosticsPlugin},
    prelude::*,
};
use reactor_ui::sickle::prelude::*;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntrySystem;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryCpuUsageLabel;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryCpuUsageData;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryMemUsageLabel;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryMemUsageData;

impl PerfUiEntry for PerfUiEntrySystem {
    fn setup(app: &mut App) {
        app.add_plugins(SystemInformationDiagnosticsPlugin);
        app.add_systems(
            Update,
            (update_cpu_usage, update_mem_usage).in_set(ReactorPerfUiSchedule::Update),
        );
    }
    fn spawn(list: &mut UiBuilder<Entity>) {
        let config = PanelEntryCollapsibleConfig {
            label: "Engine".into(),
        };

        list.panel_entry_collapsible(config, |collapse| {
            collapse.panel_entry_text(PanelEntryTextConfig {
                title_text: "CPU Usage:".to_string(),
                title_component: PerfUiEntryCpuUsageLabel,
                content_text: "X %".to_string(),
                content_component: PerfUiEntryCpuUsageData,
            });

            collapse.panel_entry_text(PanelEntryTextConfig {
                title_text: "Mem Usage:".to_string(),
                title_component: PerfUiEntryMemUsageLabel,
                content_text: "X %".to_string(),
                content_component: PerfUiEntryMemUsageData,
            });
        });

        list.insert(PerfUiEntrySystem);
    }
}

fn update_cpu_usage(
    mut text_children: Query<&mut Text, With<PerfUiEntryCpuUsageData>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    for mut text in text_children.iter_mut() {
        let usage = diagnostics
            .get(&SystemInformationDiagnosticsPlugin::CPU_USAGE)
            .and_then(bevy::diagnostic::Diagnostic::smoothed)
            .unwrap_or(0.0);
        text.sections[0].value = format!("{} %", utils::format_pretty_float(2, 2, usage));
    }
}

fn update_mem_usage(
    mut text_children: Query<&mut Text, With<PerfUiEntryMemUsageData>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    for mut text in text_children.iter_mut() {
        let usage = diagnostics
            .get(&SystemInformationDiagnosticsPlugin::MEM_USAGE)
            .and_then(bevy::diagnostic::Diagnostic::smoothed)
            .unwrap_or(0.0);
        text.sections[0].value = format!("{} %", utils::format_pretty_float(2, 2, usage));
    }
}
