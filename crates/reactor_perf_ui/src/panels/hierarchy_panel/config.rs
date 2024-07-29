use bevy::prelude::*;

/// Configuration for the performance panel.
#[derive(Resource, Debug, Clone)]
pub struct HierarchyPanelConfig {
    /// Show the entity list.
    pub entity_list: bool,
    /// Show the resources list.
    pub(crate) _resources_list: bool,
}

impl HierarchyPanelConfig {
    /// Creates a new instance of the Performance panel config. With everything turned on.
    pub fn full() -> Self {
        Default::default()
    }
}

impl Default for HierarchyPanelConfig {
    fn default() -> Self {
        Self {
            entity_list: true,
            _resources_list: false,
        }
    }
}
