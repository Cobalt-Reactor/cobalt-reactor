use crate::{prelude::*, sickle::prelude::*};

/// Calls the callback when the widget is clicked (pressed and released).
/// Note: This has exclusive World access and thus can't be used in parallel with other systems.
/// That means that if you are doing a lot of stuff, or doing things every frame it's going to get
/// expensive Use with caution.
pub trait StyleWIthPositionExt<'a> {
    /// Calls `callback` when the widget is clicked.
    fn with_position(&mut self, position: &ReactorPosition) -> &mut UiStyle<'a>;
}

impl<'a> StyleWIthPositionExt<'a> for UiStyle<'a> {
    fn with_position(&mut self, position: &ReactorPosition) -> &mut UiStyle<'a> {
        self.position_type(position.position_type);

        if let Some(left) = position.left {
            self.left(left);
        }

        if let Some(right) = position.right {
            self.right(right);
        }

        if let Some(top) = position.top {
            self.top(top);
        }

        if let Some(bottom) = position.bottom {
            self.bottom(bottom);
        }

        self
    }
}
