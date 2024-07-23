use crate::{events::CloseCobalt, CobaltState};
use bevy::prelude::*;

pub fn handle_close_cobalt(
    mut state: ResMut<NextState<CobaltState>>,
    reader: EventReader<CloseCobalt>,
) {
    if !reader.is_empty() {
        state.set(CobaltState::Exiting);
    }
}

pub fn close_on_esc(mut writer: EventWriter<CloseCobalt>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        writer.send(CloseCobalt);
    }
}
