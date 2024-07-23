use super::{
    exiting::plugin::ExitingStatePlugin, loading::plugin::LoadingStatePlugin,
    running::plugin::RunningStatePlugin,
};
use bevy::prelude::*;

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        Self::add_plugins(app);
    }
}

impl StatesPlugin {
    pub fn add_plugins(app: &mut App) {
        app.add_plugins(ExitingStatePlugin)
            .add_plugins(LoadingStatePlugin)
            .add_plugins(RunningStatePlugin);
    }
}
