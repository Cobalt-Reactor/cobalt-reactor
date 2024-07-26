use bevy::prelude::Val;

/// Configuration for anything with a size.
#[derive(Default, Debug, Clone)]
pub struct ReactorSize {
    /// The width of the object.
    pub width: ReactorSizeType,
    /// The height of the object.
    pub height: ReactorSizeType,
}

/// Size type, Set takes a val and sets the size to that value.
/// Stretch sets the size to stretch based on the content.
#[derive(Default, Debug, Clone)]
pub enum ReactorSizeType {
    /// Sets a concrete value.
    Set(ReactorSizeConfig),
    /// Sets the size to stretch based on the content.
    #[default]
    Fill,
}

impl From<ReactorSizeConfig> for ReactorSizeType {
    fn from(config: ReactorSizeConfig) -> Self {
        Self::Set(config)
    }
}

impl From<Val> for ReactorSizeType {
    fn from(val: Val) -> Self {
        Self::Set(val.into())
    }
}

/// Configuration for concrete sized objects.
#[derive(Default, Debug, Clone)]
pub struct ReactorSizeConfig {
    /// The base size of the object.
    pub base: Val,
    /// The maximum size of the object.
    pub max: Val,
    /// The minimum size of the object.
    pub min: Val,
}

impl From<Val> for ReactorSizeConfig {
    fn from(val: Val) -> Self {
        Self {
            base: val,
            max: val,
            min: val,
        }
    }
}

impl From<(Val, Val, Val)> for ReactorSizeConfig {
    fn from((base, min, max): (Val, Val, Val)) -> Self {
        Self { base, min, max }
    }
}
