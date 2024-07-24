use bevy::prelude::Val;

/// Configuration anything with a size.
#[derive(Default, Debug)]
pub struct ReactorSize {
    /// The width of the object.
    pub width: Val,
    /// The height of the object.
    pub height: Val,
}
