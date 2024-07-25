use crate::prelude::*;

/// Basic configuration for all widgets
#[derive(Default, Debug, Clone)]
pub struct ReactorBaseConfig {
    /// The position of the widget.
    pub position: ReactorPosition,
    /// The size of the widget.
    pub size: ReactorSize,
    /// The alignment of the widget.
    pub alignment: ReactorAlignment,
    /// Whether the widget is pickable
    pub picking: Option<ReactorPicking>,
}
