use crate::prelude::*;
use bevy::prelude::*;

/// Extension trait for adding new types of Perf UI Entries.
pub(crate) trait PerfUiAppExt {
    /// Adds a new Perf UI widget to the application.
    fn add_perf_ui_widget<W: ReactorPerfUiPanel<Config = C>, C: Resource + Clone>(
        &mut self,
        config: C,
    ) -> &mut Self;
}

impl PerfUiAppExt for App {
    fn add_perf_ui_widget<W: ReactorPerfUiPanel<Config = C>, C: Resource + Clone>(
        &mut self,
        config: C,
    ) -> &mut Self {
        self.insert_resource(config.clone());

        W::setup(self, config.clone());

        self.add_systems(
            Update,
            spawn_widget::<W, C>.in_set(ReactorPerfUiSchedule::Spawn),
        );

        self
    }
}

pub(crate) fn spawn_widget<W: ReactorPerfUiPanel<Config = C>, C: Resource + Clone>(
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
