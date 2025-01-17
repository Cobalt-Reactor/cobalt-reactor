use super::*;
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
        if !app.is_plugin_added::<SystemInformationDiagnosticsPlugin>() {
            app.add_plugins(SystemInformationDiagnosticsPlugin);
        }

        app.add_systems(
            Update,
            (update_cpu_usage, update_mem_usage).in_set(ReactorPerfUiSchedule::Update),
        );

        app.register_type::<PerfUiEntrySystem>()
            .register_type::<PerfUiEntryCpuUsageLabel>()
            .register_type::<PerfUiEntryTimeFrameCountData>()
            .register_type::<PerfUiEntryCpuUsageData>()
            .register_type::<PerfUiEntryMemUsageLabel>()
            .register_type::<PerfUiEntryMemUsageData>();
    }

    fn spawn(list: &mut UiBuilder<Entity>) {
        let config = collapsible_header_config("System".into());

        list.list_item_collapsible_header(config, |collapse| {
            collapse.list_item_two_text(PanelEntryTextConfig {
                title_text: "CPU Usage:".to_string(),
                title_component: PerfUiEntryCpuUsageLabel,
                content_text: "X %".to_string(),
                content_component: PerfUiEntryCpuUsageData,
            });

            collapse.list_item_two_text(PanelEntryTextConfig {
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
