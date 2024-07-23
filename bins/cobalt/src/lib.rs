//! Core lib for cobalt
mod cobalt_state;
pub(crate) use cobalt_state::CobaltState;
mod cobalt;
pub use cobalt::Cobalt;
mod debug;
/// Defines
pub(crate) mod defines;
mod render;
mod states;

/// Plugins
pub(crate) mod plugins {
    pub use crate::{
        debug::plugin::DebugPlugin, render::plugin::RenderPlugin, states::plugin::StatesPlugin,
    };
}

pub(crate) mod schedules {
    pub use crate::states::schedule::StateHandlingSchedule;
}

pub(crate) mod assets {}

pub(crate) mod events {
    pub use crate::states::events::*;
}

pub(crate) mod resources {
    pub use crate::render::resources::*;
}

pub(crate) mod components {
    pub use crate::render::components::*;
}

// use bevy::prelude::*;
//
// pub struct TemplatePlugin;
//
// impl Plugin for TemplatePlugin {
//     fn build(&self, app: &mut App) {
//         self.add_events(app);
//         self.add_plugins(app);
//         self.register_types(app);
//         self.insert_resources(app);
//         self.add_systems(app);
//         self.configure_sets(app);
//     }
// }
//
// impl TemplatePlugin {
//     pub fn add_events(&self, _: &mut App) {}
//
//     pub fn add_plugins(&self, _: &mut App) {}
//
//     pub fn register_types(&self, _: &mut App) {}
//
//     pub fn insert_resources(&self, _: &mut App) {}
//
//     pub fn add_systems(&self, _: &mut App) {}
//
//     pub fn configure_sets(&self, _: &mut App) {}
// }
