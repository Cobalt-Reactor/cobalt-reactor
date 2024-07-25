use super::PerfUiEntry;
use crate::utils::next_sort_key;
use bevy::{diagnostic::DiagnosticsStore, ecs::system::lifetimeless::SRes, prelude::*};

#[derive(Component, Debug, Clone)]
pub struct PerfUiEntryFps {
    /// Sort Key (control where the entry will appear in the Perf UI).
    pub sort_key: i32,
}

impl PerfUiEntry for PerfUiEntryFps {
    fn setup(_: &mut App) {}

    type SystemParamUpdate = SRes<DiagnosticsStore>;

    fn spawn(_: &mut reactor_ui::sickle::prelude::UiBuilder<Entity>) {}

    fn update(
        _: Entity,
        _: &mut <Self::SystemParamUpdate as bevy::ecs::system::SystemParam>::Item<'_, '_>,
    ) {
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }
}

impl Default for PerfUiEntryFps {
    fn default() -> Self {
        Self {
            sort_key: next_sort_key(),
        }
    }
}
