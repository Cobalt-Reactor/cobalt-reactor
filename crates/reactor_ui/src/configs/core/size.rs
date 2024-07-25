use bevy::prelude::Val;

/// Configuration anything with a size.
#[derive(Default, Debug, Clone)]
pub struct ReactorSize {
    /// The width of the object.
    pub width: Val,
    /// The height of the object.
    pub height: Val,
}
