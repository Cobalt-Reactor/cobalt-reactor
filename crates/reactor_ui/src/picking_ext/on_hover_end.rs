use crate::picking::*;
use bevy::{ecs::system::EntityCommands, prelude::*};

/// Calls the callback when the widget is hovered over, when that hover ends.
/// Note: This has exclusive World access and thus can't be used in parallel with other systems.
/// That means that if you are doing a lot of stuff, or doing things every frame it's going to get
/// expensive Use with caution.
pub trait UiOnHoverEndExt<'a> {
    /// Calls `callback` when the widget is hovered over, when that hover ends.
    fn on_hover_end<Marker>(
        &mut self,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> &mut EntityCommands<'a>;

    /// Emits the event when the widget is hovered over, when that hover ends.
    fn on_hover_end_event<F: Event + From<ListenerInput<Pointer<Out>>>>(
        &mut self,
    ) -> &mut EntityCommands<'a>;
}

impl<'a> UiOnHoverEndExt<'a> for EntityCommands<'a> {
    fn on_hover_end<Marker>(
        &mut self,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> &mut EntityCommands<'a> {
        self.insert(On::<Pointer<Out>>::run(callback))
    }

    fn on_hover_end_event<F: Event + From<ListenerInput<Pointer<Out>>>>(
        &mut self,
    ) -> &mut EntityCommands<'a> {
        self.insert(On::<Pointer<Out>>::send_event::<F>())
    }
}
