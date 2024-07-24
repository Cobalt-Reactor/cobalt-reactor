#[cfg(debug_assertions)]
pub mod plugin {
    use super::systems::setup_fps_counter;
    use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
    use iyes_perf_ui::prelude::PerfUiPlugin;

    pub struct DebugPlugin;

    impl Plugin for DebugPlugin {
        fn build(&self, app: &mut App) {
            Self::add_plugins(app);
            Self::add_systems(app);
        }
    }

    impl DebugPlugin {
        pub fn add_plugins(app: &mut App) {
            app.add_plugins(FrameTimeDiagnosticsPlugin)
                .add_plugins(PerfUiPlugin);
        }

        pub fn add_systems(app: &mut App) {
            app.add_systems(Startup, setup_fps_counter);
        }
    }
}

mod systems {
    use bevy::prelude::*;
    use iyes_perf_ui::prelude::*;

    pub fn setup_fps_counter(mut commands: Commands) {
        commands.spawn((
            PerfUiRoot {
                display_labels: true,
                ..default()
            },
            PerfUiEntryFPSWorst::default(),
            PerfUiEntryFPS::default(),
        ));
    }
}
