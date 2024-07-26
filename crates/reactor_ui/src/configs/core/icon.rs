use crate::prelude::*;
use bevy::prelude::*;

/// Configuration for an image source.
#[derive(Debug, Clone)]
pub enum ReactorIconSource {
    /// A path to an image file.
    Image(ReactorImageSource),
}

impl From<ReactorImageSource> for ReactorIconSource {
    fn from(source: ReactorImageSource) -> Self {
        ReactorIconSource::Image(source)
    }
}

impl From<Handle<Image>> for ReactorIconSource {
    fn from(source: Handle<Image>) -> Self {
        ReactorIconSource::Image(source.into())
    }
}
