use crate::picking::*;
use bevy::{ecs::system::EntityCommands, prelude::*};

/// Emits the event each frame while the widget is being dragged.
pub trait UiWhileDraggedExt<'a> {
    /// Emits the event each frame while the widget is being dragged.
    fn while_dragged<F: Event + From<ListenerInput<Pointer<Drag>>>>(
        &mut self,
    ) -> &mut EntityCommands<'a>;
}

impl<'a> UiWhileDraggedExt<'a> for EntityCommands<'a> {
    fn while_dragged<F: Event + From<ListenerInput<Pointer<Drag>>>>(
        &mut self,
    ) -> &mut EntityCommands<'a> {
        self.insert(On::<Pointer<Drag>>::send_event::<F>())
    }
}
