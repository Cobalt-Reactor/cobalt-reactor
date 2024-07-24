//! Core lib for cobalt
pub use cobalt::Cobalt;
pub(crate) use states::cobalt_state::CobaltState;

mod cobalt;
mod debug;
mod render;
mod states;
mod ui;

pub(crate) mod defines;

pub(crate) mod plugins {
    pub use crate::{
        debug::plugin::DebugPlugin, render::plugin::RenderPlugin, states::plugin::StatesPlugin,
        ui::plugin::UiPlugin,
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
//     fn add_events(&self, _: &mut App) {}
//
//     fn add_plugins(&self, _: &mut App) {}
//
//     fn register_types(&self, _: &mut App) {}
//
//     fn insert_resources(&self, _: &mut App) {}
//
//     fn add_systems(&self, _: &mut App) {}
//
//     fn configure_sets(&self, _: &mut App) {}
// }
