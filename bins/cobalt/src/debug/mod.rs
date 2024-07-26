#[cfg(debug_assertions)]
pub mod plugin {
    use bevy::prelude::*;

    pub struct DebugPlugin;

    impl Plugin for DebugPlugin {
        fn build(&self, app: &mut App) {
            Self::add_plugins(app);
            Self::add_systems(app);
        }
    }

    impl DebugPlugin {
        pub fn add_plugins(_: &mut App) {}

        pub fn add_systems(_: &mut App) {}
    }
}
