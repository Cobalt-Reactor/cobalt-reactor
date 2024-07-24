use crate::picking::*;
use bevy::{ecs::system::EntityCommands, prelude::*};

/// Calls the callback when the widget is clicked, on button release
/// Note: This has exclusive World access and thus can't be used in parallel with other systems.
/// That means that if you are doing a lot of stuff, or doing things every frame it's going to get
/// expensive Use with caution.
pub trait UiOnReleasedExt<'a> {
    /// Creates a button.
    fn on_released<Marker>(
        &mut self,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> &mut EntityCommands<'a>;
}

impl<'a> UiOnReleasedExt<'a> for EntityCommands<'a> {
    fn on_released<Marker>(
        &mut self,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> &mut EntityCommands<'a> {
        self
            .insert(On::<Pointer<Down>>::run(callback))
    }
}
