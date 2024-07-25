use crate::sickle::prelude::*;
use bevy::prelude::*;

/// Configuration for an image source.
#[derive(Debug, Clone)]
pub enum ReactorImageBackground {
    /// A path to an image file.
    Path(String),
    /// A path to an image file and a layout for a texture atlas.
    Atlas(String, ReactorAtlasLayout),
}

/// Configuration a texture atlas.
#[derive(Default, Debug, Clone)]
pub struct ReactorAtlasLayout {
    /// The size of each tile in the atlas.
    pub tile_size: UVec2,
    /// The number of columns in the atlas.
    pub columns: u32,
    /// The number of rows in the atlas.
    pub rows: u32,
    /// The padding between each tile.
    pub padding: Option<UVec2>,
    /// The offset of the atlas within the image.
    pub offset: Option<UVec2>,
}

impl From<ReactorImageBackground> for ImageSource {
    fn from(source: ReactorImageBackground) -> Self {
        match source {
            ReactorImageBackground::Path(path) => ImageSource::Path(path),
            ReactorImageBackground::Atlas(path, layout) => ImageSource::Atlas(path, layout.into()),
        }
    }
}

impl From<&ReactorImageBackground> for ImageSource {
    fn from(source: &ReactorImageBackground) -> Self {
        match source {
            ReactorImageBackground::Path(path) => ImageSource::Path(path.to_string()),
            ReactorImageBackground::Atlas(path, layout) => {
                ImageSource::Atlas(path.to_string(), layout.into())
            }
        }
    }
}

impl From<ReactorAtlasLayout> for TextureAtlasLayout {
    fn from(layout: ReactorAtlasLayout) -> Self {
        TextureAtlasLayout::from_grid(
            layout.tile_size,
            layout.columns,
            layout.rows,
            layout.padding,
            layout.offset,
        )
    }
}

impl From<&ReactorAtlasLayout> for TextureAtlasLayout {
    fn from(layout: &ReactorAtlasLayout) -> Self {
        TextureAtlasLayout::from_grid(
            layout.tile_size,
            layout.columns,
            layout.rows,
            layout.padding,
            layout.offset,
        )
    }
}

impl From<&str> for ReactorImageBackground {
    fn from(path: &str) -> Self {
        Self::Path(path.to_string())
    }
}

impl From<String> for ReactorImageBackground {
    fn from(path: String) -> Self {
        Self::Path(path)
    }
}
