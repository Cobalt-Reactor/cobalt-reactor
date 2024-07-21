use bevy::prelude::*;

/// Trait for building modules. A module is a set of functionality that can be applied to an entity
/// For example, a module might be used to apply movement to an entity where movement requires
/// multiple components or bundles.
///
/// `install` is called when the module is added to an entity using `EntityBuilder::build`
/// `update` is available to update the module on active entities at run time,
/// this is useful for hot reloading, and is called by `EntityBuilder::update`
pub trait Module: Send + Sync {
    /// Installs the module on the target entity
    fn install(&self, target: &mut EntityWorldMut);
    /// Updates the module on the target entity, useful for hot reloading
    fn update(&self, _: &mut EntityWorldMut);
}
