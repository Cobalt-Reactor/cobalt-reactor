use super::PerfUiEntry;
use crate::{prelude::*, utils};
use bevy::{
    self,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PrimaryWindow,
};
use reactor_ui::{prelude::*, sickle::prelude::*};

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryWindow;

#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryWindowResolutionLabel;

#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryWindowResolutionData;

#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryWindowScaleLabel;

#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryWindowScaleData;

#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryWindowModeLabel;

#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryWindowModeData;

#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryWindowPresentModeLabel;

#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryWindowPresentModeData;

impl PerfUiEntry for PerfUiEntryWindow {
    fn setup(app: &mut App) {
        app.add_systems(
            Update,
            (
                update_resolution,
                update_scale_factor,
                update_window_mode,
                update_present_mode,
            )
                .in_set(ReactorPerfUiSchedule::Update),
        );
    }
    fn spawn(list: &mut reactor_ui::sickle::prelude::UiBuilder<Entity>) {
        let config = ListItemCollapsibleConfig {
            label: "Window".into(),
        };

        list.insert(PerfUiEntryWindow);

        list.list_item_collapsible(config, |collapse| {
            collapse.list_item_two_text(ListItemTwoTextConfig {
                title_text: "Resolution:".to_string(),
                title_component: PerfUiEntryWindowResolutionLabel,
                content_text: "X x Y".to_string(),
                content_component: PerfUiEntryWindowResolutionData,
            });

            collapse.list_item_two_text(ListItemTwoTextConfig {
                title_text: "Scale Factor:".to_string(),
                title_component: PerfUiEntryWindowScaleLabel,
                content_text: "99.99".to_string(),
                content_component: PerfUiEntryWindowScaleData,
            });

            collapse.list_item_two_text(ListItemTwoTextConfig {
                title_text: "Window Mode:".to_string(),
                title_component: PerfUiEntryWindowModeLabel,
                content_text: "DEFAULT".to_string(),
                content_component: PerfUiEntryWindowModeData,
            });

            collapse.list_item_two_text(ListItemTwoTextConfig {
                title_text: "Present Mode:".to_string(),
                title_component: PerfUiEntryWindowPresentModeLabel,
                content_text: "DEFAULT".to_string(),
                content_component: PerfUiEntryWindowPresentModeData,
            });
        });
    }
}

fn update_resolution(
    mut text_children: Query<&mut Text, With<PerfUiEntryWindowResolutionData>>,
    primary_windows: Query<&Window, (With<PrimaryWindow>, Or<(Added<Window>, Changed<Window>)>)>,
) {
    if primary_windows.is_empty() {
        return;
    }

    for mut text in text_children.iter_mut() {
        let window = primary_windows.get_single().unwrap();
        let width = window.width();
        let height = window.height();
        text.sections[0].value = format!("{} x {}", width, height);
    }
}

fn update_scale_factor(
    mut text_children: Query<&mut Text, With<PerfUiEntryWindowScaleData>>,
    primary_windows: Query<&Window, (With<PrimaryWindow>, Or<(Added<Window>, Changed<Window>)>)>,
) {
    if primary_windows.is_empty() {
        return;
    }

    for mut text in text_children.iter_mut() {
        let scale = primary_windows.get_single().unwrap().scale_factor();
        text.sections[0].value = utils::format_pretty_float(3, 0, scale.into());
    }
}

fn update_window_mode(
    mut text_children: Query<&mut Text, With<PerfUiEntryWindowModeData>>,
    primary_windows: Query<&Window, (With<PrimaryWindow>, Or<(Added<Window>, Changed<Window>)>)>,
) {
    if primary_windows.is_empty() {
        return;
    }

    for mut text in text_children.iter_mut() {
        let mode = match primary_windows.get_single().unwrap().mode {
            bevy::window::WindowMode::Windowed => "Windowed".to_string(),
            bevy::window::WindowMode::BorderlessFullscreen => "Borderless Full".to_string(),
            bevy::window::WindowMode::SizedFullscreen => "Sized Full".to_string(),
            bevy::window::WindowMode::Fullscreen => "Fullscreen".to_string(),
        };

        text.sections[0].value = mode;
    }
}

fn update_present_mode(
    mut text_children: Query<&mut Text, With<PerfUiEntryWindowPresentModeData>>,
    primary_windows: Query<&Window, (With<PrimaryWindow>, Or<(Added<Window>, Changed<Window>)>)>,
) {
    if primary_windows.is_empty() {
        return;
    }

    for mut text in text_children.iter_mut() {
        let mode = match primary_windows.get_single().unwrap().present_mode {
            bevy::window::PresentMode::AutoVsync => "AutoVsync".to_string(),
            bevy::window::PresentMode::AutoNoVsync => "AutoNoVsync".to_string(),
            bevy::window::PresentMode::Fifo => "Fifo".to_string(),
            bevy::window::PresentMode::FifoRelaxed => "FifoRelaxed".to_string(),
            bevy::window::PresentMode::Immediate => "Immediate".to_string(),
            bevy::window::PresentMode::Mailbox => "Mailbox".to_string(),
        };

        text.sections[0].value = mode;
    }
}
