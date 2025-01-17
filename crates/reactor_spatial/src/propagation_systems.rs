use crate::prelude::*;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
/// System schedule for 2D spatial propagation
pub enum SpatialSystems2D {
    /// Propagate 2D spatial components to Bevys transform components
    Propagate,
}

pub fn propagate_spatial2d(
    mut query: Query<(
        &mut Transform,
        &Position2D,
        &Rotation2D,
        &Scale2D,
        &DrawOrder,
        &RotationPropagation,
        &PositionPropagation,
        &ScalePropagation,
        Option<&Parent>,
    )>,
    all_parent_rotations: Query<&Rotation2D, With<Children>>,
    all_parent_positions: Query<&Position2D, With<Children>>,
    all_parent_scales: Query<&Scale2D, With<Children>>,
) {
    query.par_iter_mut().for_each(
        |(mut transform, position, rotation, scale, draw_order, r_prop, p_prop, s_prop, parent)| {
            let mut new_rot = Quat::from(rotation);
            let mut new_pos = Vec3::new(position.x, position.y, draw_order.into());
            let mut new_scale = Vec3::new(scale.x, scale.y, 1.0);

            if let Some(parent) = parent {
                let parent_rotation = all_parent_rotations.get(parent.get()).unwrap();
                let parent_position = all_parent_positions.get(parent.get()).unwrap();
                let parent_scale = all_parent_scales.get(parent.get()).unwrap();

                if r_prop == &RotationPropagation::Absolute {
                    // Counteract parent's rotation effects on position
                    let temp = position.rotate_radians(-parent_rotation.radians());
                    new_pos.x = temp.x;
                    new_pos.y = temp.y;
                    // Counteract parent's rotation
                    new_rot = Quat::from(-parent_rotation);
                }

                if p_prop == &PositionPropagation::Absolute {
                    // Counteract parent's position
                    new_pos.x = -parent_position.x;
                    new_pos.y = -parent_position.y;
                }

                if s_prop == &ScalePropagation::Absolute {
                    // Counteract parent's scale
                    new_scale.x = 1.0 / parent_scale.x;
                    new_scale.y = 1.0 / parent_scale.y;
                }
            }

            transform.rotation = new_rot;
            transform.translation = new_pos;
            transform.scale = new_scale;
        },
    );
}

pub fn update_compass_from_rotation2d(mut query: Query<(&mut Compass, &Rotation2D)>) {
    query.par_iter_mut().for_each(|(mut compass, rotation)| {
        *compass = Compass::from(rotation);
    });
}

pub fn update_compass_rose_from_rotation2d(mut query: Query<(&mut CompassRose, &Rotation2D)>) {
    query.par_iter_mut().for_each(|(mut compass, rotation)| {
        *compass = rotation.into();
    });
}

pub fn update_compass_halfwinds_from_rotation2d(
    mut query: Query<(&mut CompassHalfwinds, &Rotation2D)>,
) {
    query.par_iter_mut().for_each(|(mut compass, rotation)| {
        *compass = rotation.into();
    });
}
