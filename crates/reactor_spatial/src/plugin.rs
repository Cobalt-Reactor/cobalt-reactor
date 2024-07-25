use crate::prelude::*;
use bevy::prelude::*;

/// The core plugin for `reactor_spatial`, add this to your app
pub struct ReactorSpatialPlugin;

impl Plugin for ReactorSpatialPlugin {
    fn build(&self, app: &mut App) {
        self.add_events(app);
        self.add_plugins(app);
        self.register_types(app);
        self.insert_resources(app);
        self.add_systems(app);
        self.configure_sets(app);
    }
}

impl ReactorSpatialPlugin {
    /// Creates a new `ReactorSpatialPlugin`
    pub fn new() -> Self {
        Default::default()
    }

    fn add_events(&self, _: &mut App) {}

    fn add_plugins(&self, _: &mut App) {}

    fn register_types(&self, app: &mut App) {
        app.register_type::<DrawOrder>()
            .register_type::<RotationPropagation>()
            .register_type::<PositionPropagation>()
            .register_type::<ScalePropagation>()
            .register_type::<Position2D>()
            .register_type::<Rotation2D>()
            .register_type::<Scale2D>()
            .register_type::<Degrees>()
            .register_type::<Radians>()
            .register_type::<Compass>()
            .register_type::<CompassHalfwinds>()
            .register_type::<CompassRose>();
    }

    fn insert_resources(&self, _: &mut App) {}

    fn add_systems(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                propagate_spatial2d,
                update_compass_from_rotation2d,
                update_compass_halfwinds_from_rotation2d,
                update_compass_rose_from_rotation2d,
            )
                .in_set(SpatialSystems2D::Propagate)
                .before(TransformSystem::TransformPropagate),
        )
        .add_systems(
            PostStartup,
            (
                propagate_spatial2d,
                update_compass_from_rotation2d,
                update_compass_halfwinds_from_rotation2d,
                update_compass_rose_from_rotation2d,
            )
                .in_set(SpatialSystems2D::Propagate)
                .before(TransformSystem::TransformPropagate),
        );
    }

    fn configure_sets(&self, _: &mut App) {}
}

impl Default for ReactorSpatialPlugin {
    fn default() -> Self {
        Self
    }
}
