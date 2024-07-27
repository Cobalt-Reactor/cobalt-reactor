use crate::module::Module;
use bevy::{
    ecs::system::{EntityCommand, EntityCommands},
    prelude::*,
};

/// An entity builder, used to build entities from modules
pub struct EntityBuilder<'a> {
    target: &'a mut EntityBuilderTarget<'a>,
}

pub enum EntityBuilderTarget<'a> {
    EntityCommands(&'a mut EntityCommands<'a>),
    EntityWorldMut(&'a mut EntityWorldMut<'a>),
}

impl<'a> EntityBuilder<'a> {
    /// Creates a new entity builder
    pub fn new(target: &'a mut EntityBuilderTarget<'a>) -> Self {
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
        };

        self
    }
}

// TODO: `entity_builder` extension for EntityCommands and the CommandQueue equiv that
// returns an entity builder for the given entity
// TODO: Create a common modules crate for many of the modules in Zombies (and refactor hearing and
// vision to have optional Fixed or Unlimited range)
