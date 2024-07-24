use crate::picking::*;
use bevy::{ecs::system::EntityCommands, prelude::*};

/// Calls the callback when an object that is being dragged enters the widget.
/// Note: This has exclusive World access and thus can't be used in parallel with other systems.
/// That means that if you are doing a lot of stuff, or doing things every frame it's going to get
/// expensive Use with caution.
pub trait UiOnDraggableEnteredExt<'a> {
    /// Calls `callback` when an object that is being dragged enters the widget.
    fn on_draggable_entered<Marker>(
        &mut self,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> &mut EntityCommands<'a>;
}

impl<'a> UiOnDraggableEnteredExt<'a> for EntityCommands<'a> {
    fn on_draggable_entered<Marker>(
        &mut self,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> &mut EntityCommands<'a> {
        self.insert(On::<Pointer<DragEnter>>::run(callback))
    }
}
