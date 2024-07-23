//! Core lib for cobalt
mod cobalt_state;
pub(crate) use cobalt_state::CobaltState;
mod cobalt;
pub use cobalt::Cobalt;
mod debug;
/// Defines
pub(crate) mod defines;
mod states;

/// Plugins
pub(crate) mod plugins {
    pub use crate::{debug::plugin::DebugPlugin, states::plugin::StatesPlugin};
}

pub(crate) mod schedules {
    pub use crate::states::schedule::StateHandlingSchedule;
}

pub(crate) mod assets {}

pub(crate) mod events {
    pub use crate::states::events::*;
}
