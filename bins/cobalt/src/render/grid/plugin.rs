use super::{defines::*, rendering::*, systems::*};
use crate::components::*;
use bevy::{asset::load_internal_asset, prelude::*, sprite::Material2dPlugin};
use std::marker::PhantomData;

/// The plugin which allows floor grids to work, where `T` is the component to track the floor grid
/// to
pub struct TrackedGridPlugin<T: Component> {
    _phantom: PhantomData<T>,
}

impl<T: Component> TrackedGridPlugin<T> {
    /// Adds the plugin along with a default floor grid
    pub const fn with_floor_grid() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    /// Adds the plugin without spawning a default floor grid
    pub const fn without_floor_grid() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    fn add_plugins(&self, app: &mut App) {
        app.add_plugins((
            Material2dPlugin::<SimpleLineMaterial>::default(),
            Material2dPlugin::<ClippedLineMaterial>::default(),
        ));
    }

    fn add_systems(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (main_grid_mesher, sub_grid_mesher, grid_axis_mesher),
        )
        .add_systems(
            Update,
            (
                despawn_children_upon_removal::<Grid, GridChild>,
                despawn_children_upon_removal::<Grid, SubGridChild>,
                despawn_children_upon_removal::<Grid, GridAxisChild>,
                despawn_children_upon_removal::<SubGrid, SubGridChild>,
                despawn_children_upon_removal::<GridAxis, GridAxisChild>,
            ),
        );
    }

    fn load_assets(&self, app: &mut App) {
        load_internal_asset!(
            app,
            CLIPPED_LINE_SHADER_HANDLE,
            "shaders/clipped_line.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            SIMPLE_LINE_SHADER_HANDLE,
            "shaders/simple_line.wgsl",
            Shader::from_wgsl
        );
    }

    fn register_types(&self, app: &mut App) {
        app.register_type::<Grid>()
            .register_type::<GridAxis>()
            .register_type::<SubGrid>()
            .register_type::<GridChild>()
            .register_type::<GridAxisChild>()
            .register_type::<SubGridChild>()
            .register_type::<GridAlignment>();
    }
}

impl<T: Component> Default for TrackedGridPlugin<T> {
    fn default() -> Self {
        Self::with_floor_grid()
    }
}

impl<T: Component> Plugin for TrackedGridPlugin<T> {
    fn build(&self, app: &mut App) {
        self.load_assets(app);
        self.register_types(app);
        self.add_plugins(app);
        self.add_systems(app);
    }
}

pub type GridPlugin = TrackedGridPlugin<Camera>;
