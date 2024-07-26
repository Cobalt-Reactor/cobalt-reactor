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
            background_color: Some(Color::Srgba(tailwind::GRAY_700)),
            corner_radius: Some(ReactorCornerRadius::from(10.0)),
            border_config: Some(ReactorBorder {
                border_color: Color::Srgba(tailwind::GRAY_900),
                border_width: UiRect::all(Val::Px(2.0)),
            }),
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
