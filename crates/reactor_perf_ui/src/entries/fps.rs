use super::PerfUiEntry;
use bevy::{
    diagnostic::DiagnosticsStore,
    ecs::system::{lifetimeless::SRes, SystemParam},
    prelude::*,
};
use reactor_ui::sickle::prelude::*;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryFps;

impl PerfUiEntry for PerfUiEntryFps {
    fn setup(_: Commands) {}

    type SystemParamUpdate = SRes<DiagnosticsStore>;

    fn spawn(row: &mut UiBuilder<Entity>) {
        row.label(LabelConfig {
            label: "FPS".into(),
            ..default()
        })
        .style()
        .height(Val::Px(20.0))
        .entity_commands()
        .insert(PerfUiEntryFps);
    }

    fn update(_: Entity, _: &mut <Self::SystemParamUpdate as SystemParam>::Item<'_, '_>) {}
}
