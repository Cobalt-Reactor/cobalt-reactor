use crate::module::Module;
use bevy::prelude::*;

/// An entity builder, used to build entities from modules
#[derive(Default)]
pub struct EntityBuilder {
    modules: Vec<Box<dyn Module>>,
}

impl EntityBuilder {
    /// Creates a new entity builder
    pub fn new() -> Self {
        Default::default()
    }

    /// Adds a module to the builder
    pub fn with<M: Module + 'static>(mut self, module: M) -> Self {
        self.modules.push(Box::new(module));
        self
    }

    /// Installs all added modules on the target
    pub fn build(self, target: &mut EntityWorldMut) {
        for module in self.modules {
            module.install(target);
        }
    }

    /// Updates all added modules, useful for hot reloading
    #[cfg(feature = "hot_reload")]
    pub fn rebuild(self, target: &mut EntityWorldMut) {
        for module in self.modules {
            module.update(target);
        }
    }
}
