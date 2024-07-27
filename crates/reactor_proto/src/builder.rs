use crate::module::Module;
use bevy::{ecs::system::EntityCommands, prelude::*};

/// An entity builder, used to build entities from modules
pub struct EntityBuilder<'a> {
    target: EntityBuilderTarget<'a>,
}

pub enum EntityBuilderTarget<'a> {
    Id(Entity, Commands<'a, 'a>),
    Commands(&'a mut EntityCommands<'a>),
    WorldMut(&'a mut EntityWorldMut<'a>),
}

impl<'a> EntityBuilder<'a> {
    /// Creates a new entity builder
    pub fn new(target: EntityBuilderTarget<'a>) -> Self {
        Self { target }
    }

    /// Adds a module to the associated entity
    pub fn install<M: Module + 'static>(&mut self, module: M) -> &Self {
        match &mut self.target {
            EntityBuilderTarget::Commands(ref mut ec) => {
                ec.add(move |mut ewm: EntityWorldMut| module.install(&mut ewm));
            }
            EntityBuilderTarget::WorldMut(ref mut ewm) => {
                module.install(ewm);
            }
            EntityBuilderTarget::Id(id, commands) => {
                if let Some(mut entity) = commands.get_entity(*id) {
                    entity.add(move |mut ewm: EntityWorldMut| {
                        module.install(&mut ewm);
                    });
                }
            }
        };

        self
    }

    /// Adds a module to the builder
    #[cfg(feature = "hot_reload")]
    pub fn update<M: Module + 'static>(&mut self, module: M) -> &Self {
        match &mut self.target {
            EntityBuilderTarget::Commands(ref mut ec) => {
                ec.add(move |mut ewm: EntityWorldMut| module.update(&mut ewm));
            }
            EntityBuilderTarget::WorldMut(ref mut ewm) => {
                module.update(ewm);
            }
            EntityBuilderTarget::Id(id, commands) => {
                if let Some(mut entity) = commands.get_entity(*id) {
                    entity.add(move |mut ewm: EntityWorldMut| {
                        module.update(&mut ewm);
                    });
                }
            }
        };

        self
    }
}

/// Gets a builder for an entity
pub trait EntityBuilderExt<'a> {
    /// Gets a builder for an entity
    fn entity_builder(&'a mut self) -> EntityBuilder<'a>;
}

impl<'a> EntityBuilderExt<'a> for EntityCommands<'a> {
    fn entity_builder(&'a mut self) -> EntityBuilder<'a> {
        EntityBuilder::new(EntityBuilderTarget::Commands(self))
    }
}

impl<'a> EntityBuilderExt<'a> for EntityWorldMut<'a> {
    fn entity_builder(&'a mut self) -> EntityBuilder<'a> {
        EntityBuilder::new(EntityBuilderTarget::WorldMut(self))
    }
}

impl<'a> EntityBuilderExt<'a> for Commands<'a, 'a> {
    fn entity_builder(&'a mut self) -> EntityBuilder<'a> {
        let id = self.spawn_empty().id();

        EntityBuilder::new(EntityBuilderTarget::Id(id, self.reborrow()))
    }
}

// TODO: `entity_builder` extension for EntityCommands and the CommandQueue equiv that
// returns an entity builder for the given entity
// TODO: Create a common modules crate for many of the modules in Zombies (and refactor hearing and
// vision to have optional Fixed or Unlimited range)
