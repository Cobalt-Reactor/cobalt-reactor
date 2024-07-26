use crate::prelude::*;
use bevy::{
    ecs::system::{EntityCommand, EntityCommands},
    prelude::*,
};

impl EntityCommand for ReactorFontConfig {
    fn apply(self, entity: Entity, world: &mut World) {
        let font = match self.font {
            ReactorFontType::Path(path) => {
                let asset_server = world.resource::<AssetServer>();
                Some(asset_server.load(&path))
            }
            ReactorFontType::Handle(handle) => Some(handle),
            ReactorFontType::BuiltIn => None,
        };

        if let Some(mut text) = world.entity_mut(entity).get_mut::<Text>() {
            for text_section in &mut text.sections {
                if let Some(font) = font.clone() {
                    text_section.style.font = font;
                }
                text_section.style.font_size = self.size;
                text_section.style.color = self.color;
            }
        }
    }
}

/// Adds a font to the entity. font is the path to the font asset. This file can be loaded ahead of
/// time if you want to avoid loads during runtime.
pub trait UiFontCommandExt<'a> {
    /// Adds a font to the entity.
    fn with_font(&'a mut self, font: ReactorFontConfig) -> &mut EntityCommands<'a>;
}

impl<'a> UiFontCommandExt<'a> for EntityCommands<'a> {
    fn with_font(&'a mut self, font: ReactorFontConfig) -> &mut EntityCommands<'a> {
        self.add(font)
    }
}
