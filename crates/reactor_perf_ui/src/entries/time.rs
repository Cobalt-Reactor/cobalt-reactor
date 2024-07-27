use super::PerfUiEntry;
use crate::{prelude::*, utils};
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use reactor_ui::sickle::prelude::*;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryTime;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryTimeFrameCountLabel;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryTimeFrameCountData;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryRunningTimeLabel;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryRunningTimeData;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryClockTimeLabel;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryClockTimeData;

impl PerfUiEntry for PerfUiEntryTime {
    fn setup(app: &mut App) {
        if !app.is_plugin_added::<FrameTimeDiagnosticsPlugin>() {
            app.add_plugins(FrameTimeDiagnosticsPlugin);
        }
        app.add_systems(
            Update,
            (update_clock_time, update_frame_count, update_running_time)
                .in_set(ReactorPerfUiSchedule::Update),
        );
    }
    fn spawn(list: &mut UiBuilder<Entity>) {
        let config = PanelEntryCollapsibleConfig {
            label: "Time".into(),
        };
        list.panel_entry_collapsible(config, |collapse| {
            collapse.panel_entry_text(PanelEntryTextConfig {
                title_text: "Wall Time:".to_string(),
                title_component: PerfUiEntryClockTimeLabel,
                content_text: "X".to_string(),
                content_component: PerfUiEntryClockTimeData,
            });

            collapse.panel_entry_text(PanelEntryTextConfig {
                title_text: "Running Time:".to_string(),
                title_component: PerfUiEntryRunningTimeLabel,
                content_text: "X".to_string(),
                content_component: PerfUiEntryRunningTimeData,
            });

            collapse.panel_entry_text(PanelEntryTextConfig {
                title_text: "Frame Count:".to_string(),
                title_component: PerfUiEntryTimeFrameCountLabel,
                content_text: "X".to_string(),
                content_component: PerfUiEntryTimeFrameCountData,
            });
        });

        list.insert(PerfUiEntryTime);
    }
}

fn update_clock_time(mut text_children: Query<&mut Text, With<PerfUiEntryClockTimeData>>) {
    for mut text in text_children.iter_mut() {
        #[cfg(feature = "chrono")]
        let time = get_system_clock_local();
        #[cfg(not(feature = "chrono"))]
        let time = get_system_clock_utc();

        if let Some((h, m, s, n)) = time {
            text.sections[0].value = utils::format_pretty_time_hms(2, h, m, s, n);
        }
    }
}

fn update_running_time(
    mut text_children: Query<&mut Text, With<PerfUiEntryRunningTimeData>>,
    time: Res<Time<Fixed>>,
) {
    for mut text in text_children.iter_mut() {
        let time = time.elapsed();
        text.sections[0].value = utils::format_pretty_time(2, time);
    }
}

fn update_frame_count(
    mut text_children: Query<&mut Text, With<PerfUiEntryTimeFrameCountData>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    for mut text in text_children.iter_mut() {
        let count = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FRAME_COUNT)
            .and_then(bevy::diagnostic::Diagnostic::smoothed)
            .unwrap_or(0.0) as i64;
        text.sections[0].value = utils::format_pretty_int(6, count);
    }
}

#[cfg(feature = "chrono")]
fn get_system_clock_local() -> Option<(u32, u32, u32, u32)> {
    use chrono::Timelike;
    let now = chrono::Local::now();
    let h = now.hour();
    let m = now.minute();
    let s = now.second();
    let nanos = now.timestamp_subsec_nanos();
    Some((h, m, s, nanos))
}

#[cfg(not(feature = "chrono"))]
fn get_system_clock_utc() -> Option<(u32, u32, u32, u32)> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?;
    let secs = now.as_secs();
    let h = (secs / 3600) % 24;
    let m = (secs / 60) % 60;
    let s = secs % 60;
    let nanos = now.subsec_nanos();
    Some((h as u32, m as u32, s as u32, nanos))
}
