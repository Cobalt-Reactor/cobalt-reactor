use super::PerfUiEntry;
use crate::{prelude::*, utils};
use bevy::{
    self,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use reactor_ui::{prelude::*, sickle::prelude::*};

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

impl PerfUiEntry for PerfUiEntryFps {
    fn setup(app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin);
        app.add_systems(
            Update,
            (Self::update_fps, Self::update_fps_worst).in_set(ReactorPerfUiSchedule::Update),
        );
    }

    fn spawn(list: &mut UiBuilder<Entity>) {
        let config = PanelEntryCollapsibleConfig {
            label: "FPS".into(),
        };

        list.insert(PerfUiEntryFps)
            .style()
            .background_color(Color::Srgba(tailwind::GREEN_700));

        // Add a collapsible List Item here with other list items in it
        list.panel_entry_collapsible(config, |collapse| {
            collapse.panel_entry_text(PanelEntryTextConfig {
                title_text: "FPS (avg):".to_string(),
                title_component: PerfUiEntryFpsAvgLabel,
                content_text: "1.0".to_string(),
                content_component: PerfUiEntryFpsAvgData,
            });

            collapse.panel_entry_text(PanelEntryTextConfig {
                title_text: "FPS (worst):".to_string(),
                title_component: PerfUiEntryFpsWorstLabel,
                content_text: "1.0".to_string(),
                content_component: PerfUiEntryFpsWorstData::default(),
            });
        });
    }
}

impl PerfUiEntryFps {
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
}
