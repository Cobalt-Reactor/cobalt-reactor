use super::PerfUiEntry;
use bevy::prelude::*;

#[derive(Component, Debug, Clone, Default)]
pub struct PerfUiEntryWindow;

impl PerfUiEntry for PerfUiEntryWindow {
    fn setup(_: &mut App) {}
    fn spawn(_: &mut reactor_ui::sickle::prelude::UiBuilder<Entity>) {}
}
