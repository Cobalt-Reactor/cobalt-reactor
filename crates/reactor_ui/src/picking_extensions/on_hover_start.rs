use crate::picking::*;
use bevy::{ecs::system::EntityCommands, prelude::*};

/// Calls the callback when the widget is hovered over, when that hover starts.
/// Note: This has exclusive World access and thus can't be used in parallel with other systems.
/// That means that if you are doing a lot of stuff, or doing things every frame it's going to get
/// expensive Use with caution.
pub trait UiOnHoverStartExt<'a> {
    /// Calls `callback` when the widget is hovered over and that hover starts.
    fn on_hover_start<Marker>(
        &mut self,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> &mut EntityCommands<'a>;
}

impl<'a> UiOnHoverStartExt<'a> for EntityCommands<'a> {
    fn on_hover_start<Marker>(
        &mut self,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> &mut EntityCommands<'a> {
        self.insert(On::<Pointer<Over>>::run(callback))
    }
}
