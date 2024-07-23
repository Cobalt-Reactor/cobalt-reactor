use crate::{components::*, resources::MouseWorldCoords};
use bevy::{
    color::palettes::tailwind, prelude::*, render::camera::ScalingMode, window::PrimaryWindow,
};
use reactor_core::{camera::*, spatial::Position2D};

pub fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(CameraBundle2D {
            style: CameraStyle::Exact,
            lerp: CameraLerp::new(10.),
            camera_bundle: Camera2dBundle {
                projection: OrthographicProjection {
                    scaling_mode: ScalingMode::WindowSize(1.0),
                    near: -1000.,
                    far: 1000.,
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .insert(MainCamera);
}

pub fn create_grid(mut commands: Commands) {
    commands.spawn((
        Grid {
            spacing: 32.0,
            count: 32,
            color: Color::Srgba(tailwind::GRAY_900),
            alpha_mode: AlphaMode::Opaque,
        },
        GridAxis::new_rgb(),
        TransformBundle::default(),
        VisibilityBundle::default(),
        Name::new("Grid"),
    ));
}

pub fn mouse_to_world(
    mut mouse_coords: ResMut<MouseWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    windows: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, cam_xform) = cameras.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = windows.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_pos) = window
        .cursor_position()
        .and_then(|mouse_pos| camera.viewport_to_world(cam_xform, mouse_pos))
        .map(|ray| ray.origin.truncate())
    {
        mouse_coords.0 = Position2D::from(world_pos);
    }
}
