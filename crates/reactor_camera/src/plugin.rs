use crate::prelude::*;
use bevy::prelude::*;
use reactor_spatial::prelude::*;

/// The core plugin for `reactor_camera`, add this to your app
pub struct ReactorCameraPlugin;

impl Plugin for ReactorCameraPlugin {
    fn build(&self, app: &mut App) {
        self.register_types(app);
        self.add_systems(app);
    }
}

impl ReactorCameraPlugin {
    /// Creates a new `ReactorCameraPlugin`
    pub fn new() -> Self {
        Default::default()
    }

    fn register_types(&self, app: &mut App) {
        app.register_type::<MainCamera>()
            .register_type::<MainCameraShouldTarget>()
            .register_type::<CameraStyle>()
            .register_type::<DeadZone>()
            .register_type::<CameraLerp>()
            .register_type::<CameraLead>()
            .register_type::<CameraTarget>();
    }

    fn add_systems(&self, app: &mut App) {
        app.add_systems(
            PostStartup,
            set_initial_main_camera_target.before(SpatialSystems2D::Propagate),
        )
        .add_systems(
            PostUpdate,
            (
                camera_follow_target,
                main_camera_target_added,
                main_camera_target_removed,
            )
                .before(SpatialSystems2D::Propagate),
        );
    }
}

impl Default for ReactorCameraPlugin {
    fn default() -> Self {
        Self
    }
}
