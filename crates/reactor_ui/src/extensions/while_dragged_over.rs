use crate::picking::*;
use bevy::{ecs::system::EntityCommands, prelude::*};

/// Emits the event each frame while an object that is being dragged is over the widget.
pub trait UiWhileDraggedOverExt<'a> {
    /// Emits the event each frame while an object that is being dragged is over the widget.
    fn while_dragged_over<F: Event + From<ListenerInput<Pointer<DragOver>>>>(
        &mut self,
    ) -> &mut EntityCommands<'a>;
}

impl<'a> UiWhileDraggedOverExt<'a> for EntityCommands<'a> {
    fn while_dragged_over<F: Event + From<ListenerInput<Pointer<DragOver>>>>(
        &mut self,
    ) -> &mut EntityCommands<'a> {
        self.insert(On::<Pointer<DragOver>>::send_event::<F>())
    }
}
