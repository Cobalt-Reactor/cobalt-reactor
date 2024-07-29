use crate::prelude::*;
use bevy::{asset::io::embedded::EmbeddedAssetRegistry, prelude::*};

/// The main plugin for the Reactor Perf UI crate. Add this to your app to enable the Reactor Perf
/// UIs.
#[derive(Default)]
pub struct ReactorPerfUiPlugin {
    start_visible: bool,
    perf_panel: Option<PerfPanelConfig>,
    hierarchy_panel: Option<HierarchyPanelConfig>,
}

impl Plugin for ReactorPerfUiPlugin {
    fn build(&self, app: &mut App) {
        self.add_events(app);
        self.add_plugins(app);
        self.register_types(app);
        self.insert_resources(app);
        self.add_systems(app);
        self.configure_sets(app);
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

    /// Enables the Perf Panel (fps, cpu usage, etc).
    pub fn with_perf_panel(mut self) -> Self {
        self.perf_panel = Some(PerfPanelConfig::default());
        self
    }

    /// Enables the Perf Panel (fps, cpu usage, etc).
    pub fn with_perf_panel_config(mut self, config: PerfPanelConfig) -> Self {
        self.perf_panel = Some(config);
        self
    }

    /// Enables the entity hierarchy panel
    pub fn with_hierarchy_panel(mut self) -> Self {
        self.hierarchy_panel = Some(HierarchyPanelConfig::default());
        self
    }

    /// Enables the entity hierarchy panel
    pub fn with_hierarchy_panel_config(mut self, config: HierarchyPanelConfig) -> Self {
        self.hierarchy_panel = Some(config);
        self
    }

    fn add_events(&self, _: &mut App) {}

    fn add_plugins(&self, _: &mut App) {}

    fn register_types(&self, _: &mut App) {}

    fn insert_resources(&self, _: &mut App) {}

    fn add_systems(&self, _: &mut App) {}

    fn configure_sets(&self, _: &mut App) {}

    fn add_widgets(&self, app: &mut App) {
        if let Some(perf_panel) = &self.perf_panel {
            app.add_panel::<ReactorPerfPanel, _>(perf_panel.clone());
        }

        if let Some(hierarchy_panel) = &self.hierarchy_panel {
            app.add_panel::<ReactorHierarchyPanel, _>(hierarchy_panel.clone());
        }
    }
}
