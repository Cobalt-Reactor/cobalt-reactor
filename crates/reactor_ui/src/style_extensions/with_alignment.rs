use crate::{prelude::*, sickle::prelude::*};

/// Calls the callback when the widget is clicked (pressed and released).
/// Note: This has exclusive World access and thus can't be used in parallel with other systems.
/// That means that if you are doing a lot of stuff, or doing things every frame it's going to get
/// expensive Use with caution.
pub trait StyleWithAlignmentExt<'a> {
    /// Adds all alignment to the widget.
    fn with_alignment(&mut self, alignment: &ReactorAlignment) -> &mut UiStyle<'a>;
    /// Adds self alignment to the widget.
    fn with_self_alignment(&mut self, alignment: &ReactorSelfAlignment) -> &mut UiStyle<'a>;
    /// Adds child alignment to the widget.
    fn with_child_alignment(&mut self, alignment: &ReactorChildAlignment) -> &mut UiStyle<'a>;
}

impl<'a> StyleWithAlignmentExt<'a> for UiStyle<'a> {
    fn with_alignment(&mut self, alignment: &ReactorAlignment) -> &mut UiStyle<'a> {
        if let Some(self_align) = &alignment.self_alignment {
            self.with_self_alignment(self_align);
        }

        if let Some(child_align) = &alignment.child_alignment {
            self.with_child_alignment(child_align);
        }

        self
    }

    fn with_self_alignment(&mut self, alignment: &ReactorSelfAlignment) -> &mut UiStyle<'a> {
        if let Some(align) = alignment.horizontal {
            self.justify_self(align);
        }

        if let Some(align) = alignment.vertical {
            self.align_self(align);
        }

        self
    }

    fn with_child_alignment(&mut self, alignment: &ReactorChildAlignment) -> &mut UiStyle<'a> {
        if let Some(align) = alignment.horizontal {
            self.justify_items(align);
        }

        if let Some(align) = alignment.vertical {
            self.align_items(align);
        }

        if let Some(align) = alignment.horizontal_distribution {
            self.justify_content(align);
        }

        if let Some(align) = alignment.vertical_distribution {
            self.align_content(align);
        }

        self
    }
}
