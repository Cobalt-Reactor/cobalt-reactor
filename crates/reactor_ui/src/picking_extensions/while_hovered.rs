use crate::picking::*;
use bevy::{ecs::system::EntityCommands, prelude::*};

/// Emits the event each frame while the widget is being hovered over.
pub trait UiWhileHoveredExt<'a> {
    /// Emits the event each frame while the widget is being hovered over.
    fn while_hovered<F: Event + From<ListenerInput<Pointer<Move>>>>(
        &mut self,
    ) -> &mut EntityCommands<'a>;
}

impl<'a> UiWhileHoveredExt<'a> for EntityCommands<'a> {
    fn while_hovered<F: Event + From<ListenerInput<Pointer<Move>>>>(
        &mut self,
    ) -> &mut EntityCommands<'a> {
        self.insert(On::<Pointer<Move>>::send_event::<F>())
    }
}
