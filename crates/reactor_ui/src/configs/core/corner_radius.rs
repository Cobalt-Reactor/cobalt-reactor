use bevy::prelude::*;

/// Configuration anything with a size.
#[derive(Default, Debug, Clone)]
pub struct ReactorCornerRadius {
    /// The top left corner radius.
    pub tl: f32,
    /// The top right corner radius.
    pub tr: f32,
    /// The bottom left corner radius.
    pub bl: f32,
    /// The bottom right corner radius.
    pub br: f32,
}

impl From<f32> for ReactorCornerRadius {
    fn from(radius: f32) -> Self {
        Self {
            tl: radius,
            tr: radius,
            bl: radius,
            br: radius,
        }
    }
}

impl From<(f32, f32, f32, f32)> for ReactorCornerRadius {
    fn from((tl, tr, bl, br): (f32, f32, f32, f32)) -> Self {
        Self { tl, tr, bl, br }
    }
}

impl From<(f32, f32)> for ReactorCornerRadius {
    fn from((horizontal, vertical): (f32, f32)) -> Self {
        Self {
            tl: vertical,
            tr: horizontal,
            bl: vertical,
            br: horizontal,
        }
    }
}

impl From<ReactorCornerRadius> for BorderRadius {
    fn from(radius: ReactorCornerRadius) -> Self {
        Self {
            top_left: Val::Px(radius.tl),
            top_right: Val::Px(radius.tr),
            bottom_left: Val::Px(radius.bl),
            bottom_right: Val::Px(radius.br),
        }
    }
}

impl From<&ReactorCornerRadius> for BorderRadius {
    fn from(radius: &ReactorCornerRadius) -> Self {
        Self {
            top_left: Val::Px(radius.tl),
            top_right: Val::Px(radius.tr),
            bottom_left: Val::Px(radius.bl),
            bottom_right: Val::Px(radius.br),
        }
    }
}
