use crate::prelude::*;
use bevy::prelude::*;

/// Extension trait for adding new types of Perf UI Entries.
pub(crate) trait PerfUiAppExt {
    /// Adds a new Perf UI widget to the application.
    fn add_perf_ui_widget<W: ReactorPerfUiWidget<Config = C>, C: Resource + Clone>(
        &mut self,
        config: C,
    ) -> &mut Self;
}

impl PerfUiAppExt for App {
    fn add_perf_ui_widget<W: ReactorPerfUiWidget<Config = C>, C: Resource + Clone>(
        &mut self,
        config: C,
    ) -> &mut Self {
        self.insert_resource(config.clone());

        self.add_systems(
            Update,
            setup_widget::<W, C>.in_set(ReactorPerfUiSchedule::Setup),
        );

        self.add_systems(
            Update,
            spawn_widget::<W, C>.in_set(ReactorPerfUiSchedule::Spawn),
        );

        self
    }
}

pub(crate) fn setup_widget<W: ReactorPerfUiWidget<Config = C>, C: Resource + Clone>(
    commands: Commands,
    config: Res<C>,
    mut setup_complete: Local<bool>,
) {
    if *setup_complete {
        return;
    }
    W::setup(commands, config);
    *setup_complete = true;
}

pub(crate) fn spawn_widget<W: ReactorPerfUiWidget<Config = C>, C: Resource + Clone>(
    commands: Commands,
    config: Res<C>,
    mut spawn_complete: Local<bool>,
) {
    if *spawn_complete {
        return;
    }
    W::spawn(commands, config);
    *spawn_complete = true;
}
