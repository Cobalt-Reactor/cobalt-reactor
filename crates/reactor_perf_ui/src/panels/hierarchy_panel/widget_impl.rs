use crate::{entries::*, prelude::*};
use bevy::prelude::*;
use reactor_ui::{prelude::*, sickle::prelude::*};

impl ReactorPerfUiPanel for ReactorHierarchyPanel {
    type Config = HierarchyPanelConfig;

    fn setup(_app: &mut App, _config: Self::Config) {}

    fn spawn(mut commands: Commands, _config: Res<Self::Config>) {
        commands
            .ui_builder(UiRoot)
            .floating_window(perf_panel_ui_config(), |window| {
                window.growable_list(ReactorGrowableListConfig::default(), |_list| {});
            });
    }
}

fn perf_panel_ui_config() -> ReactorFloatingWindowConfig {
    ReactorFloatingWindowConfig {
        draggable: true,
        size: ReactorSize {
            width: Val::Px(290.0).into(),
            height: Val::Auto.into(),
        },
        position: ReactorPosition::Absolute(ReactorPositionAbsolute { x: 328.0, y: 32.0 }),
        background: ReactorBackground::Flat(ReactorFlatBackground {
            background_color: Some(Color::Srgba(tailwind::GRAY_600)),
            corner_radius: Some(ReactorCornerRadius::from(10.0)),
            border_config: Some(ReactorBorder {
                border_color: Color::Srgba(tailwind::GRAY_900),
                border_width: UiRect::all(Val::Px(2.0)),
            }),
        }),
        header_config: Some(ReactorHeaderConfig {
            label: "Hierarchy".into(),
            font: Some(ReactorFontConfig {
                size: 16.0,
                color: Color::Srgba(tailwind::GRAY_50),
                font: fonts::STD.into(),
            }),
        }),
    }
}
