use crate::prelude::*;
#[cfg(feature = "progress_tracking")]
use crate::systems::track_progress;
use bevy::prelude::*;
#[cfg(feature = "progress_tracking")]
use iyes_progress::{ProgressCounter, ProgressSystem, TrackedProgressSet};

/// Plugin for all of `rantz_proto`. Add this to your app.
pub struct ProtoPlugin;

impl Plugin for ProtoPlugin {
    fn build(&self, app: &mut App) {
        Self::init_resources(app);
        Self::add_systems(app);
    }
}

impl ProtoPlugin {
    fn init_resources(app: &mut App) {
        app.init_resource::<ManifestLoader>();
    }

    fn add_systems(app: &mut App) {
        app.add_systems(
            PostUpdate,
            handle_async_spawn.in_set(ProtoSchedule::Spawning),
        );
        #[cfg(feature = "progress_tracking")]
        app.add_systems(
            Update,
            track_progress
                .track_progress()
                .run_if(resource_exists::<ProgressCounter>)
                .in_set(TrackedProgressSet)
                .in_set(ProtoSchedule::Loading),
        );
    }
}
