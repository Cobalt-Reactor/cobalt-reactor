use crate::{prelude::*, sickle::prelude::*};
use bevy::prelude::*;

/// Calls the callback when the widget is clicked (pressed and released).
/// Note: This has exclusive World access and thus can't be used in parallel with other systems.
/// That means that if you are doing a lot of stuff, or doing things every frame it's going to get
/// expensive Use with caution.
pub trait StyleWithPositionExt<'a> {
    /// Calls `callback` when the widget is clicked.
    fn with_position(&mut self, position: &ReactorPosition) -> &mut UiStyle<'a>;
}

impl<'a> StyleWithPositionExt<'a> for UiStyle<'a> {
    fn with_position(&mut self, position: &ReactorPosition) -> &mut UiStyle<'a> {
        info!("Position: {:?}", position);
        match position {
            ReactorPosition::Absolute(pos) => {
                self.position_type(PositionType::Absolute);
                self.absolute_position(Vec2::new(pos.x, pos.y))
            }
            ReactorPosition::Relative(pos) => {
                self.position_type(PositionType::Relative);

                self.left(pos.left);
                self.right(pos.right);
                self.top(pos.top);
                self.bottom(pos.bottom);

                self
            }
        }
    }
}
