use bevy::{ecs::system::EntityCommands, ui::*};
use sickle_ui::prelude::*;

/// Makes a widget draggable.
/// Note: This has exclusive World access and thus can't be used in parallel with other systems.
/// That means that if you are doing a lot of stuff, or doing things every frame it's going to get
/// expensive Use with caution.
pub trait UiDraggableExt<'a> {
    /// Creates a button.
    fn draggable(&mut self) -> &mut EntityCommands<'a>;
}

impl<'a> UiDraggableExt<'a> for EntityCommands<'a> {
    fn draggable(&mut self) -> &mut EntityCommands<'a> {
        self.insert((
            TrackedInteraction::default(),
            Draggable::default(),
            RelativeCursorPosition::default(),
        ))
    }
}
