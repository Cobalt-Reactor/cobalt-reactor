use super::PerfUiEntry;
use crate::{prelude::*, utils};
use bevy::{
    diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin},
    prelude::*,
};
use reactor_ui::{prelude::*, sickle::prelude::*};

pub(crate) fn collapsible_header_config(text: String) -> ReactorCollapsibleConfig {
    ReactorCollapsibleConfig {
        open: true,
        background: ReactorBackground::Flat(ReactorFlatBackground {
            background_color: Some(Color::Srgba(tailwind::GRAY_700)),
            border_config: Some(ReactorBorder {
                border_width: UiRect {
                    bottom: Val::Px(2.0),
                    ..default()
                },
                border_color: Color::Srgba(tailwind::GRAY_800),
            }),
            ..default()
        }),
        size: ReactorSize {
            width: Val::Percent(100.0).into(),
            height: ReactorSizeType::Set(Val::Px(20.0).into()),
        },
        label: ReactorTextLabelConfig {
            label: text.into(),
            font: Some(entry_header_font()),
            ..default()
        },
        expand_icon: icons::EXPAND.into(),
        collapse_icon: icons::COLLAPSE.into(),
    }
}

pub(crate) fn entity_entry_config(text: String) -> ReactorCollapsibleConfig {
    ReactorCollapsibleConfig {
        open: false,
        background: ReactorBackground::Flat(ReactorFlatBackground {
            background_color: Some(Color::Srgba(tailwind::GRAY_600)),
            border_config: Some(ReactorBorder {
                border_width: UiRect {
                    bottom: Val::Px(2.0),
                    ..default()
                },
                border_color: Color::Srgba(tailwind::GRAY_800),
            }),
            ..default()
        }),
        size: ReactorSize {
            width: Val::Percent(100.0).into(),
            height: ReactorSizeType::Set(Val::Px(16.0).into()),
        },
        label: ReactorTextLabelConfig {
            label: text.into(),
            font: Some(entry_content_font()),
            ..default()
        },
        expand_icon: icons::EXPAND.into(),
        collapse_icon: icons::COLLAPSE.into(),
    }
}

pub(crate) fn list_item_config() -> ReactorListItemConfig {
    ReactorListItemConfig {
        background: ReactorBackground::Flat(ReactorFlatBackground {
            border_config: Some(ReactorBorder {
                border_color: Color::Srgba(tailwind::GRAY_900),
                border_width: UiRect::bottom(Val::Px(2.0)),
            }),
            ..default()
        }),
        size: ReactorSize {
            height: Val::Px(20.0).into(),
            ..default()
        },
    }
}

pub(crate) fn entry_content_font() -> ReactorFontConfig {
    ReactorFontConfig {
        font: fonts::STD.into(),
        size: 12.0,
        color: Color::Srgba(tailwind::GRAY_100),
    }
}

pub(crate) fn entry_label_font() -> ReactorFontConfig {
    ReactorFontConfig {
        font: fonts::STD.into(),
        size: 12.0,
        color: Color::Srgba(tailwind::GRAY_100),
    }
}

pub(crate) fn entry_header_font() -> ReactorFontConfig {
    ReactorFontConfig {
        size: 15.0,
        color: Color::Srgba(tailwind::GRAY_100),
        font: fonts::STD.into(),
    }
}

pub(crate) fn panel_header_font() -> ReactorFontConfig {
    ReactorFontConfig {
        size: 18.0,
        color: Color::Srgba(tailwind::GRAY_100),
        font: fonts::STD.into(),
    }
}
