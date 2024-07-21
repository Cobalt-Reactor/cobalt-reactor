use crate::prelude::*;
use bevy::prelude::*;
use reactor_spatial::prelude::*;

/// The core plugin for `reactor_camera`, add this to your app
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        self.register_types(app);
        self.add_systems(app);
    }
}

impl CameraPlugin {
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
