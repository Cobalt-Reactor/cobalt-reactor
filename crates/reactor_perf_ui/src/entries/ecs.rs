use super::PerfUiEntry;
use bevy::{diagnostic::DiagnosticsStore, ecs::system::lifetimeless::SRes, prelude::*};

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryEcs;

impl PerfUiEntry for PerfUiEntryEcs {
    fn setup(_: Commands) {}

    type SystemParamUpdate = SRes<DiagnosticsStore>;

    fn spawn(_: &mut reactor_ui::sickle::prelude::UiBuilder<Entity>) {}

    fn update(
        _: Entity,
        _: &mut <Self::SystemParamUpdate as bevy::ecs::system::SystemParam>::Item<'_, '_>,
    ) {
    }
}
