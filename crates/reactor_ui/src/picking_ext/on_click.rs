use crate::picking::*;
use bevy::{ecs::system::EntityCommands, prelude::*};

/// Calls the callback when the widget is clicked (pressed and released).
/// Note: This has exclusive World access and thus can't be used in parallel with other systems.
/// That means that if you are doing a lot of stuff, or doing things every frame it's going to get
/// expensive Use with caution.
pub trait UiOnClickExt<'a> {
    /// Calls `callback` when the widget is clicked.
    fn on_click<Marker>(
        &mut self,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> &mut EntityCommands<'a>;

    /// Emits the event when the object is clicked.
    fn on_click_event<F: Event + From<ListenerInput<Pointer<Click>>>>(
        &mut self,
    ) -> &mut EntityCommands<'a>;
}

impl<'a> UiOnClickExt<'a> for EntityCommands<'a> {
    fn on_click<Marker>(
        &mut self,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> &mut EntityCommands<'a> {
        self.insert(On::<Pointer<Click>>::run(callback))
    }

    /// Emits the event when the object is clicked.
    fn on_click_event<F: Event + From<ListenerInput<Pointer<Click>>>>(
        &mut self,
    ) -> &mut EntityCommands<'a> {
        self.insert(On::<Pointer<Click>>::send_event::<F>())
    }
}
