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

        let widget = W::default();
        widget.setup(self, config);

        self.add_systems(Startup, spawn_widget::<W, C>);

        self
    }
}

pub(crate) fn spawn_widget<W: ReactorPerfUiWidget<Config = C>, C: Resource + Clone>(
    commands: Commands,
    config: Res<C>,
) {
    W::spawn(commands, config);
}
