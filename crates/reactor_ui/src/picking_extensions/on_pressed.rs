use crate::picking::*;
use bevy::{ecs::system::EntityCommands, prelude::*};

/// Calls the callback when the widget is clicked, on button press.
/// Note: This has exclusive World access and thus can't be used in parallel with other systems.
/// That means that if you are doing a lot of stuff, or doing things every frame it's going to get
/// expensive Use with caution.
pub trait UiOnPressedExt<'a> {
    /// Calls the callback when the widget is clicked, on button press.
    fn on_pressed<Marker>(
        &mut self,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> &mut EntityCommands<'a>;

    /// Emits the event when the widget is clicked, on button press.
    fn on_pressed_event<F: Event + From<ListenerInput<Pointer<Down>>>>(
        &mut self,
    ) -> &mut EntityCommands<'a>;
}

impl<'a> UiOnPressedExt<'a> for EntityCommands<'a> {
    fn on_pressed<Marker>(
        &mut self,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> &mut EntityCommands<'a> {
        self.insert(On::<Pointer<Down>>::run(callback))
    }

    fn on_pressed_event<F: Event + From<ListenerInput<Pointer<Down>>>>(
        &mut self,
    ) -> &mut EntityCommands<'a> {
        self.insert(On::<Pointer<Down>>::send_event::<F>())
    }
}
