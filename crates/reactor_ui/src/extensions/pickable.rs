use crate::picking::*;
use bevy::ecs::system::EntityCommands;

/// Makes a widget pickable.
/// Note: This has exclusive World access and thus can't be used in parallel with other systems.
/// That means that if you are doing a lot of stuff, or doing things every frame it's going to get
/// expensive Use with caution.
pub trait UiPickableExt<'a> {
    /// Creates a button.
    fn pickable(&mut self, should_block_lower: bool, is_hoverable: bool)
        -> &mut EntityCommands<'a>;
}

impl<'a> UiPickableExt<'a> for EntityCommands<'a> {
    fn pickable(
        &mut self,
        should_block_lower: bool,
        is_hoverable: bool,
    ) -> &mut EntityCommands<'a> {
        self.insert(Pickable {
            should_block_lower,
            is_hoverable,
        })
    }
}
