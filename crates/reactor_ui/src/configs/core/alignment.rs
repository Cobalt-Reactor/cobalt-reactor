use bevy::prelude::*;

/// Alignment configuration for widgets
#[derive(Default, Debug)]
pub struct ReactorAlignment {
    /// Self alignment.
    pub self_alignment: Option<ReactorSelfAlignment>,
    /// Children alignment.
    pub child_alignment: Option<ReactorChildAlignment>,
}

/// Configuration self alignment.
#[derive(Default, Debug)]
pub struct ReactorSelfAlignment {
    /// Horizontal alignment. Default is auto.
    pub horizontal: Option<JustifySelf>,
    /// Vertical alignment. Default is auto.
    pub vertical: Option<AlignSelf>,
}

/// Configuration child alignment.
#[derive(Default, Debug)]
pub struct ReactorChildAlignment {
    /// Horizontal alignment. Default is `Default` (i.e. no alignment).
    pub horizontal: Option<JustifyItems>,
    /// Vertical alignment. Default is `Default` (i.e. no alignment).
    pub vertical: Option<AlignItems>,
    /// Horizontal distribution. Default is `Default` (i.e. no distribution).
    pub horizontal_distribution: Option<JustifyContent>,
    /// Vertical distribution. Default is `Default` (i.e. no distribution).
    pub vertical_distribution: Option<AlignContent>,
}
