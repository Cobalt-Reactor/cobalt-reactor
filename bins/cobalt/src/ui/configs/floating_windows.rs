#![allow(dead_code)]
use bevy::{color::palettes::tailwind, prelude::*};
use reactor_core::ui::*;

pub fn floating_window_config() -> ReactorFloatingWindowConfig {
    ReactorFloatingWindowConfig {
        size: ReactorSize {
            width: Val::Px(320.0),
            height: Val::Px(768.0),
        },
        position: ReactorPosition {
            position_type: PositionType::Absolute,
            left: Some(Val::Px(32.0)),
            top: Some(Val::Px(32.0)),
            ..default()
        },
        background: ReactorBackground::Flat(ReactorFlatBackground {
            background_color: Color::Srgba(tailwind::GRAY_700),
            border_color: Color::Srgba(tailwind::GRAY_900),
            corner_radius: ReactorCornerRadius::from(10.0),
            border_width: Some(Val::Px(2.0)),
        }),
        header_config: Some(ReactorHeaderConfig {
            label: "Floating Window".into(),
            font: Some(ReactorFontConfig {
                size: 24.0,
                color: Color::Srgba(tailwind::GRAY_50),
                path: "fonts/std.ttf".into(),
            }),
        }),
    }
}
