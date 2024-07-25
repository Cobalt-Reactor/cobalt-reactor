#[cfg(debug_assertions)]
pub mod plugin {
    use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

    pub struct DebugPlugin;

    impl Plugin for DebugPlugin {
        fn build(&self, app: &mut App) {
            Self::add_plugins(app);
            Self::add_systems(app);
        }
    }

    impl DebugPlugin {
        pub fn add_plugins(app: &mut App) {
            app.add_plugins(FrameTimeDiagnosticsPlugin);
        }

        pub fn add_systems(_: &mut App) {}
    }
}
