use crate::module::Module;
use bevy::{ecs::system::EntityCommands, prelude::*};

/// An entity builder, used to build entities from modules
pub struct EntityBuilder<'a> {
    target: EntityBuilderTarget<'a>,
}

pub enum EntityBuilderTarget<'a> {
    World(Entity, &'a mut World),
    Commands(Entity, &'a mut Commands<'a, 'a>),
    EntityCommands(&'a mut EntityCommands<'a>),
    EntityWorldMut(&'a mut EntityWorldMut<'a>),
}

impl<'a> EntityBuilder<'a> {
    /// Creates a new entity builder
    pub fn new(target: EntityBuilderTarget<'a>) -> Self {
        Self { target }
    }

    /// Adds a module to the associated entity
    pub fn install<M: Module + 'static>(&mut self, module: M) -> &Self {
        match &mut self.target {
            EntityBuilderTarget::EntityCommands(ref mut ec) => {
                ec.add(move |mut ewm: EntityWorldMut| module.install(&mut ewm));
            }
            EntityBuilderTarget::EntityWorldMut(ref mut ewm) => {
                module.install(ewm);
            }
            EntityBuilderTarget::Commands(id, commands) => {
                if let Some(mut entity) = commands.get_entity(*id) {
                    entity.add(move |mut ewm: EntityWorldMut| {
                        module.install(&mut ewm);
                    });
                }
            }
            EntityBuilderTarget::World(id, world) => {
                if let Some(mut ewm) = world.get_entity_mut(*id) {
                    module.install(&mut ewm);
                }
            }
        };

        self
    }

    /// Adds a module to the builder
    #[cfg(feature = "hot_reload")]
    pub fn update<M: Module + 'static>(&mut self, module: M) -> &Self {
        match &mut self.target {
            EntityBuilderTarget::EntityCommands(ref mut ec) => {
                ec.add(move |mut ewm: EntityWorldMut| module.update(&mut ewm));
            }
            EntityBuilderTarget::EntityWorldMut(ref mut ewm) => {
                module.update(ewm);
            }
            EntityBuilderTarget::Commands(id, commands) => {
                if let Some(mut entity) = commands.get_entity(*id) {
                    entity.add(move |mut ewm: EntityWorldMut| {
                        module.update(&mut ewm);
                    });
                }
            }
            EntityBuilderTarget::World(id, world) => {
                if let Some(mut ewm) = world.get_entity_mut(*id) {
                    module.update(&mut ewm);
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
        EntityBuilder::new(EntityBuilderTarget::EntityCommands(self))
    }
}

impl<'a> EntityBuilderExt<'a> for EntityWorldMut<'a> {
    fn entity_builder(&'a mut self) -> EntityBuilder<'a> {
        EntityBuilder::new(EntityBuilderTarget::EntityWorldMut(self))
    }
}

impl<'a> EntityBuilderExt<'a> for Commands<'a, 'a> {
    fn entity_builder(&'a mut self) -> EntityBuilder<'a> {
        EntityBuilder::new(EntityBuilderTarget::Commands(self.spawn_empty().id(), self))
    }
}

impl<'a> EntityBuilderExt<'a> for World {
    fn entity_builder(&'a mut self) -> EntityBuilder<'a> {
        EntityBuilder::new(EntityBuilderTarget::World(self.spawn_empty().id(), self))
    }
}
