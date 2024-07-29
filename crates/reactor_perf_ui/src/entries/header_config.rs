use super::PerfUiEntry;
use crate::{prelude::*, utils};
use bevy::{
    diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin},
    prelude::*,
};
use reactor_ui::{prelude::*, sickle::prelude::*};

pub(crate) fn default_collapsible_header_config(text: String) -> ReactorCollapsibleConfig {
    ReactorCollapsibleConfig {
        open: true,
        background: ReactorBackground::Flat(ReactorFlatBackground {
            background_color: Some(Color::Srgba(tailwind::GRAY_700)),
            border_config: Some(ReactorBorder {
                border_width: UiRect {
                    bottom: Val::Px(2.0),
                    ..default()
                },
                border_color: Color::Srgba(tailwind::GRAY_900),
            }),
            ..default()
        }),
        size: ReactorSize {
            width: Val::Percent(100.0).into(),
            height: ReactorSizeType::Set(Val::Px(24.0).into()),
        },
        label: ReactorTextLabelConfig {
            label: text.into(),
            font: Some(ReactorFontConfig {
                size: 18.0,
                color: Color::Srgba(tailwind::GRAY_100),
                font: fonts::STD.into(),
            }),
            ..default()
        },
        expand_icon: icons::EXPAND.into(),
        collapse_icon: icons::COLLAPSE.into(),
    }
}

pub(crate) fn default_collapsible_content_config(text: String) -> ReactorCollapsibleConfig {
    ReactorCollapsibleConfig {
        open: true,
        background: ReactorBackground::Flat(ReactorFlatBackground {
            background_color: Some(Color::Srgba(tailwind::GRAY_700)),
            border_config: Some(ReactorBorder {
                border_width: UiRect {
                    bottom: Val::Px(2.0),
                    ..default()
                },
                border_color: Color::Srgba(tailwind::GRAY_900),
            }),
            ..default()
        }),
        size: ReactorSize {
            width: Val::Percent(100.0).into(),
            height: ReactorSizeType::Set(Val::Px(16.0).into()),
        },
        label: ReactorTextLabelConfig {
            label: text.into(),
            font: Some(ReactorFontConfig {
                size: 14.0,
                color: Color::Srgba(tailwind::GRAY_100),
                font: fonts::STD.into(),
            }),
            ..default()
        },
        expand_icon: icons::EXPAND.into(),
        collapse_icon: icons::COLLAPSE.into(),
    }
}
