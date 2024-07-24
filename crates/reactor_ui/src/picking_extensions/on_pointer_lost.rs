use crate::picking::*;
use bevy::{ecs::system::EntityCommands, prelude::*};

/// Calls the callback when a pointer is no longer available (e.g. unplugged, virtual pointer
/// dropped, etc.) Note: This has exclusive World access and thus can't be used in parallel with
/// other systems. That means that if you are doing a lot of stuff, or doing things every frame it's
/// going to get expensive Use with caution.
pub trait UiOnPointerLostExt<'a> {
    /// Calls the callback when a pointer is no longer available (e.g. unplugged, virtual pointer
    /// dropped, etc.)
    fn on_pointer_lost<Marker>(
        &mut self,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> &mut EntityCommands<'a>;
}

impl<'a> UiOnPointerLostExt<'a> for EntityCommands<'a> {
    fn on_pointer_lost<Marker>(
        &mut self,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> &mut EntityCommands<'a> {
        self.insert(On::<Pointer<Click>>::run(callback))
    }
}