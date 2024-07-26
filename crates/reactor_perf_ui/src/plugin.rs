use crate::prelude::*;
use bevy::{asset::io::embedded::EmbeddedAssetRegistry, prelude::*};

/// The main plugin for the Reactor Perf UI crate. Add this to your app to enable the Reactor Perf
/// UIs.
#[derive(Default)]
pub struct ReactorPerfUiPlugin {
    start_visible: bool,
    perf_panel: Option<PerfPanelConfig>,
}

impl Plugin for ReactorPerfUiPlugin {
    fn build(&self, app: &mut App) {
        self.add_events(app);
        self.add_plugins(app);
        self.register_types(app);
        self.insert_resources(app);
        self.add_systems(app);
        self.configure_sets(app);
        self.load_assets(app);
        self.add_widgets(app);
    }
}

impl ReactorPerfUiPlugin {
    /// Creates a new instance of the Reactor Perf UI plugin.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the various widgets to be visible by default.
    pub fn start_visible(mut self) -> Self {
        self.start_visible = true;
        self
    }

    /// Enables the FPS counter.
    pub fn with_perf_panel(mut self) -> Self {
        self.perf_panel = Some(PerfPanelConfig::default());
        self
    }

    /// Enables the FPS counter.
    pub fn with_perf_panel_config(mut self, config: PerfPanelConfig) -> Self {
        self.perf_panel = Some(config);
        self
    }

    fn add_events(&self, _: &mut App) {}

    fn add_plugins(&self, _: &mut App) {}

    fn register_types(&self, _: &mut App) {}

    fn insert_resources(&self, _: &mut App) {}

    fn add_systems(&self, _: &mut App) {}

    fn configure_sets(&self, _: &mut App) {}

    fn load_assets(&self, app: &mut App) {
        app.init_resource::<EmbeddedAssetRegistry>();
        crate::fonts::load(app);
        crate::icons::load(app);
    }

    fn add_widgets(&self, app: &mut App) {
        if let Some(perf_panel) = &self.perf_panel {
            app.add_perf_ui_widget::<ReactorPerfPanel, _>(perf_panel.clone());
        }
    }
}
