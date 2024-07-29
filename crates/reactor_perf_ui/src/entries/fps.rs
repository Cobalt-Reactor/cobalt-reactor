use super::{default_entry_header_config, PerfUiEntry};
use crate::{prelude::*, utils};
use bevy::{
    self,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use reactor_ui::{prelude::*, sickle::prelude::*};
use std::collections::VecDeque;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryFps;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryFpsAvgLabel;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryFpsAvgData;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryFpsWorstLabel;

#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryFpsWorstData {
    pub(crate) worst_fps: f64,
}

impl Default for PerfUiEntryFpsWorstData {
    fn default() -> Self {
        Self {
            worst_fps: f64::MAX,
        }
    }
}

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryTimeFixedTimeStepDurationLabel;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryTimeFixedTimeStepDurationData;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryTimeFixedOverstepLabel;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryTimeFixedOverstepData;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryTimeFrameTimeLabel;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryTimeFrameTimeData;

impl PerfUiEntry for PerfUiEntryFps {
    fn setup(app: &mut App) {
        if !app.is_plugin_added::<FrameTimeDiagnosticsPlugin>() {
            app.add_plugins(FrameTimeDiagnosticsPlugin);
        }
        app.add_systems(
            Update,
            (
                update_fps,
                update_fps_worst,
                update_frame_time,
                update_time_step_duration,
                update_overstep,
            )
                .in_set(ReactorPerfUiSchedule::Update),
        );
    }

    fn spawn(list: &mut UiBuilder<Entity>) {
        let config = default_entry_header_config("FPS".into());

        list.insert(PerfUiEntryFps);

        list.list_item_collapsible_header(config, |collapse| {
            collapse.list_item_two_text(ListItemTwoTextConfig {
                title_text: "FPS (avg):".to_string(),
                title_component: PerfUiEntryFpsAvgLabel,
                content_text: "1.0".to_string(),
                content_component: PerfUiEntryFpsAvgData,
            });

            collapse.list_item_two_text(ListItemTwoTextConfig {
                title_text: "FPS (worst):".to_string(),
                title_component: PerfUiEntryFpsWorstLabel,
                content_text: "1.0".to_string(),
                content_component: PerfUiEntryFpsWorstData::default(),
            });

            collapse.list_item_two_text(ListItemTwoTextConfig {
                title_text: "Frame Time (ms):".to_string(),
                title_component: PerfUiEntryTimeFrameTimeLabel,
                content_text: "1".to_string(),
                content_component: PerfUiEntryTimeFrameTimeData,
            });

            collapse.list_item_two_text(ListItemTwoTextConfig {
                title_text: "Fixed Time (ms):".to_string(),
                title_component: PerfUiEntryTimeFixedTimeStepDurationLabel,
                content_text: "1".to_string(),
                content_component: PerfUiEntryTimeFixedTimeStepDurationData,
            });

            collapse.list_item_two_text(ListItemTwoTextConfig {
                title_text: "Overstep (ms):".to_string(),
                title_component: PerfUiEntryTimeFixedOverstepLabel,
                content_text: "0".to_string(),
                content_component: PerfUiEntryTimeFixedOverstepData,
            });
        });
    }
}

fn update_fps(
    mut text_children: Query<&mut Text, With<PerfUiEntryFpsAvgData>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    for mut text in text_children.iter_mut() {
        let fps = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(bevy::diagnostic::Diagnostic::average)
            .unwrap_or(0.0);
        text.sections[0].value = utils::format_pretty_float(3, 0, fps);
    }
}

fn update_fps_worst(
    mut text_children: Query<(&mut Text, &mut PerfUiEntryFpsWorstData)>,
    diagnostics: Res<DiagnosticsStore>,
) {
    for (mut text, mut data) in text_children.iter_mut() {
        let fps = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(bevy::diagnostic::Diagnostic::average)
            .unwrap_or(0.0);

        if fps < data.worst_fps {
            data.worst_fps = fps;
            text.sections[0].value = utils::format_pretty_float(3, 0, fps);
        }
    }
}

fn update_frame_time(
    mut text_children: Query<&mut Text, With<PerfUiEntryTimeFrameTimeData>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    for mut text in text_children.iter_mut() {
        let count = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
            .and_then(bevy::diagnostic::Diagnostic::smoothed)
            .unwrap_or(0.0) as i64;
        text.sections[0].value = utils::format_pretty_int(6, count);
    }
}

fn update_time_step_duration(
    mut text_children: Query<&mut Text, With<PerfUiEntryTimeFixedTimeStepDurationData>>,
    time: Res<Time<Fixed>>,
) {
    for mut text in text_children.iter_mut() {
        let time = time.timestep().as_nanos() as f64 / 1_000_000.0;
        text.sections[0].value = utils::format_pretty_float(6, 0, time);
    }
}

fn update_overstep(
    mut text_children: Query<&mut Text, With<PerfUiEntryTimeFixedOverstepData>>,
    time: Res<Time<Fixed>>,
    mut prev: Local<VecDeque<f64>>,
) {
    if prev.is_empty() {
        *prev = VecDeque::from(vec![time.overstep_fraction_f64(); 140]);
    }

    for mut text in text_children.iter_mut() {
        prev.pop_front();
        prev.push_back(time.overstep_fraction_f64());
        let time = prev.iter().sum::<f64>() / prev.len() as f64;
        text.sections[0].value = utils::format_pretty_float(2, 2, time);
    }
}
