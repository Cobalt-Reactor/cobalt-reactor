use bevy::color::palettes::tailwind;
use reactor_ui::prelude::*;

use crate::fonts;

pub fn perf_panel_ui_config() -> ReactorFloatingWindowConfig {
    ReactorFloatingWindowConfig {
        size: ReactorSize {
            width: Val::Px(290.0).into(),
            height: Val::Px(290.0).into(),
        },
        position: ReactorPosition {
            position_type: PositionType::Absolute,
            left: Some(Val::Px(32.0)),
            top: Some(Val::Px(32.0)),
            ..Default::default()
        },
        background: ReactorBackground::Flat(ReactorFlatBackground {
            background_color: Some(Color::Srgba(tailwind::GRAY_600)),
            corner_radius: Some(ReactorCornerRadius::from(10.0)),
            border_config: Some(ReactorBorder {
                border_color: Color::Srgba(tailwind::GRAY_900),
                border_width: UiRect::all(Val::Px(2.0)),
            }),
        }),
        header_config: Some(ReactorHeaderConfig {
            label: "Performance".into(),
            font: Some(ReactorFontConfig {
                size: 16.0,
                color: Color::Srgba(tailwind::GRAY_50),
                font: fonts::STD.into(),
            }),
        }),
    }
}