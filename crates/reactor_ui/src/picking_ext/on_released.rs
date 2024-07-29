use crate::picking::*;
use bevy::{ecs::system::EntityCommands, prelude::*};

/// Calls the callback when the widget is clicked, on button release
/// Note: This has exclusive World access and thus can't be used in parallel with other systems.
/// That means that if you are doing a lot of stuff, or doing things every frame it's going to get
/// expensive Use with caution.
pub trait UiOnReleasedExt<'a> {
    /// Calls the callback when the widget is clicked, on button release.
    fn on_released<Marker>(
        &mut self,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> &mut EntityCommands<'a>;

    /// Calls the callback when the widget is clicked, on button release.
    fn on_released_event<F: Event + From<ListenerInput<Pointer<Up>>>>(
        &mut self,
    ) -> &mut EntityCommands<'a>;
}

impl<'a> UiOnReleasedExt<'a> for EntityCommands<'a> {
    fn on_released<Marker>(
        &mut self,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> &mut EntityCommands<'a> {
        self.insert(On::<Pointer<Up>>::run(callback))
    }

    fn on_released_event<F: Event + From<ListenerInput<Pointer<Up>>>>(
        &mut self,
    ) -> &mut EntityCommands<'a> {
        self.insert(On::<Pointer<Up>>::send_event::<F>())
    }
}
