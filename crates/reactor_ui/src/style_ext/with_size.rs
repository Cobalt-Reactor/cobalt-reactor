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
        match &size.width {
            ReactorSizeType::Set(width) => self
                .width(width.base)
                .max_width(width.max)
                .min_width(width.min),
            ReactorSizeType::Fill => self
                .min_width(Val::Percent(100.0))
                .justify_self(JustifySelf::Stretch),
        };

        match &size.height {
            ReactorSizeType::Set(height) => self
                .height(height.base)
                .max_height(height.max)
                .min_height(height.min),
            ReactorSizeType::Fill => self
                .min_height(Val::Percent(100.0))
                .align_self(AlignSelf::Stretch),
        };

        self
    }
}
