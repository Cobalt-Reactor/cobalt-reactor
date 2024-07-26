use crate::picking::*;
use bevy::{ecs::system::EntityCommands, prelude::*};

/// Calls the callback when an object that is being dragged leaves the widget.
/// Note: This has exclusive World access and thus can't be used in parallel with other systems.
/// That means that if you are doing a lot of stuff, or doing things every frame it's going to get
/// expensive Use with caution.
pub trait UiOnDraggableExitedExt<'a> {
    /// Calls `callback` when an object that is being dragged leaves the widget.
    fn on_draggable_exited<Marker>(
        &mut self,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> &mut EntityCommands<'a>;

    /// Emits the event when an object that is being dragged leaves the widget.
    fn on_draggable_exited_event<F: Event + From<ListenerInput<Pointer<DragLeave>>>>(
        &mut self,
    ) -> &mut EntityCommands<'a>;
}

impl<'a> UiOnDraggableExitedExt<'a> for EntityCommands<'a> {
    fn on_draggable_exited<Marker>(
        &mut self,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> &mut EntityCommands<'a> {
        self.insert(On::<Pointer<DragLeave>>::run(callback))
    }

    fn on_draggable_exited_event<F: Event + From<ListenerInput<Pointer<DragLeave>>>>(
        &mut self,
    ) -> &mut EntityCommands<'a> {
        self.insert(On::<Pointer<DragLeave>>::send_event::<F>())
    }
}
