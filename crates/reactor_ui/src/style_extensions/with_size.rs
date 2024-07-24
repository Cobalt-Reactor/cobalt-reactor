use crate::{prelude::*, sickle::prelude::*};

/// Calls the callback when the widget is clicked (pressed and released).
/// Note: This has exclusive World access and thus can't be used in parallel with other systems.
/// That means that if you are doing a lot of stuff, or doing things every frame it's going to get
/// expensive Use with caution.
pub trait StyleWithSizeExt<'a> {
    /// Calls `callback` when the widget is clicked.
    fn with_size(&mut self, size: &ReactorSize) -> &mut UiStyle<'a>;
}

impl<'a> StyleWithSizeExt<'a> for UiStyle<'a> {
    fn with_size(&mut self, size: &ReactorSize) -> &mut UiStyle<'a> {
        self.width(size.width).height(size.height)
    }
}
