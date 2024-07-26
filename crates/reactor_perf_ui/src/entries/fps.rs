use super::PerfUiEntry;
use bevy::{
    diagnostic::DiagnosticsStore,
    ecs::system::{lifetimeless::SRes, SystemParam},
    prelude::*,
};
use reactor_ui::{prelude::*, sickle::prelude::*};

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryFps;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryFpsLabel;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryFpsData;

impl PerfUiEntry for PerfUiEntryFps {
    fn setup(_: Commands) {}

    type SystemParamUpdate = SRes<DiagnosticsStore>;

    fn spawn(list: &mut UiBuilder<Entity>) {
        let config = fps_list_item_config();
        list.list_item(config.clone(), |item| {
            item.with_background(&config.background)
                .style()
                .width(Val::Percent(100.0))
                .min_width(Val::Percent(100.0))
                .entity_commands()
                .insert(PerfUiEntryFps);

            // TODO: Add a small border between columns and align content to the center vertically
            item.column(|col| {
                col.style()
                    .width(Val::Percent(50.0))
                    .min_width(Val::Percent(50.0))
                    .height(config.size.height)
                    .entity_commands()
                    .insert(PerfUiEntryFpsLabel);

                col.label(LabelConfig {
                    label: "FPS :".to_string(),
                    ..default()
                })
                .style()
                .width(Val::Percent(100.0))
                .min_width(Val::Percent(100.0))
                .height(config.size.height);
            });

            item.column(|col| {
                col.style()
                    .width(Val::Percent(50.0))
                    .min_width(Val::Percent(50.0))
                    .height(config.size.height)
                    .entity_commands()
                    .insert(PerfUiEntryFpsLabel);

                col.label(LabelConfig {
                    label: "1.0".to_string(),
                    ..default()
                })
                .style()
                .width(Val::Percent(100.0))
                .min_width(Val::Percent(100.0))
                .height(config.size.height);
            });
        });
    }

    fn update(_: Entity, _: &mut <Self::SystemParamUpdate as SystemParam>::Item<'_, '_>) {}
}

pub fn fps_list_item_config() -> ReactorListItemConfig {
    ReactorListItemConfig {
        background: ReactorBackground::Flat(ReactorFlatBackground {
            border_config: Some(ReactorBorder {
                border_color: Color::Srgba(tailwind::GRAY_900),
                border_width: UiRect::vertical(Val::Px(2.0)),
            }),
            ..default()
        }),
        size: ReactorSize {
            height: Val::Px(24.0),
            ..default()
        },
    }
}
