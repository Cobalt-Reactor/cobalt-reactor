use super::rendering::SimpleLineMaterial;
use crate::components::*;
use bevy::{
    pbr::NotShadowCaster,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages, render_resource::PrimitiveTopology, view::RenderLayers,
    },
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    utils::HashMap,
};

/// Offset on the vertical axis for a sub-grid.
/// Purely used to avoid Z-fighting.
/// The mesh is offset by it, and the mesh's transform is offset by it as well.
/// Can be any reasonable float value.
const SUB_GRID_VERTICAL_OFFSET: f32 = -0.001_f32;

/// Utility function to despawn children of a certain type.
/// Used with marker components.
fn despawn_children_of_type<T: Component>(
    commands: &mut Commands,
    parent: Entity,
    children: &Children,
    query: &Query<Entity, With<T>>,
) {
    let children = children
        .into_iter()
        .filter_map(|child| query.get(*child).ok())
        .collect::<Vec<_>>();
    commands.entity(parent).remove_children(&children);
    for child in children {
        commands.entity(child).despawn();
    }
}

/// Creates vertices for a line based on the line's size and its offset
fn line_vertices(size: f32, horizontal_offset: f32, vertical_offset: f32) -> [Vec3; 8] {
    [
        // +Y line
        Vec3::new(horizontal_offset, size, vertical_offset),
        Vec3::new(horizontal_offset, -size, vertical_offset),
        // -Y line
        Vec3::new(-horizontal_offset, size, vertical_offset),
        Vec3::new(-horizontal_offset, -size, vertical_offset),
        // +X line
        Vec3::new(size, horizontal_offset, vertical_offset),
        Vec3::new(-size, horizontal_offset, vertical_offset),
        // -X line
        Vec3::new(size, -horizontal_offset, vertical_offset),
        Vec3::new(-size, -horizontal_offset, vertical_offset),
    ]
}

/// Returns the a mesh of vertices for a main grid, along with the grid's size
fn main_grid_vertices_and_size(grid: &Grid, alignment: &GridAlignment) -> (Vec<Vec3>, f32) {
    let size = grid.count as f32 * grid.spacing;
    let vertices = (0..grid.count)
        .map(|offset| (offset + 1) as f32 * grid.spacing)
        .flat_map(|offset| line_vertices(size, offset, 0.0_f32))
        .map(|vertex| alignment.shift_vec3(vertex))
        .collect::<Vec<_>>();
    (vertices, size)
}

/// System for meshing grids
pub fn main_grid_mesher(
    mut commands: Commands,
    query_parent: Query<
        (Entity, &Grid, Option<&RenderLayers>, Option<&Children>),
        (Or<(Changed<Grid>, Changed<RenderLayers>)>,),
    >,
    query_children: Query<Entity, With<GridChild>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SimpleLineMaterial>>,
) {
    for (entity, grid, render_layers, children) in query_parent.iter() {
        let (vertices, _) = main_grid_vertices_and_size(grid, &GridAlignment::default());
        let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::all());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

        if let Some(children) = children {
            despawn_children_of_type(&mut commands, entity, children, &query_children);
        }
        commands.entity(entity).with_children(|children| {
            let mut commands = children.spawn((
                GridChild,
                NotShadowCaster,
                Name::new("GridChild"),
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(mesh)),
                    material: materials
                        .add(SimpleLineMaterial::from_color(grid.color, grid.alpha_mode)),
                    ..default()
                },
            ));
            if let Some(render_layers) = render_layers {
                commands.insert(render_layers.clone());
            }
        });
    }
}

/// System for meshing sub-grids
pub fn sub_grid_mesher(
    mut commands: Commands,
    query_parent: Query<
        (
            Entity,
            &Grid,
            &SubGrid,
            Option<&GridAlignment>,
            Option<&RenderLayers>,
            Option<&Children>,
        ),
        Or<(Changed<Grid>, Changed<SubGrid>, Changed<RenderLayers>)>,
    >,
    query_children: Query<Entity, With<SubGridChild>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut simple_materials: ResMut<Assets<SimpleLineMaterial>>,
) {
    for (entity, grid, sub_grid, alignment, render_layers, children) in query_parent.iter() {
        let size = grid.count as f32 * grid.spacing;
        let sub_spacing = grid.spacing / (sub_grid.count + 1) as f32;

        let alignment = match alignment {
            Some(alignment) => alignment,
            None => &GridAlignment::default(),
        };

        let vertices = (0..grid.count)
            .flat_map(|offset| (0..sub_grid.count).map(move |sub_offset| (offset, sub_offset)))
            .map(|(offset, sub_offset)| {
                (sub_offset as f32).mul_add(sub_spacing, offset as f32 * grid.spacing + sub_spacing)
            })
            .flat_map(|offset| line_vertices(size, offset, SUB_GRID_VERTICAL_OFFSET))
            .map(|vertex| alignment.shift_vec3(vertex))
            .collect::<Vec<_>>();

        let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::all());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

        if let Some(children) = children {
            despawn_children_of_type(&mut commands, entity, children, &query_children);
        }
        commands.entity(entity).with_children(|children| {
            let mut child_commands = children.spawn((
                SubGridChild,
                NotShadowCaster,
                VisibilityBundle::default(),
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(mesh.clone())),
                    material: simple_materials
                        .add(SimpleLineMaterial::from_color(grid.color, grid.alpha_mode)),
                    transform: Transform::from_translation(
                        alignment.shift_vec3(-Vec3::Y * SUB_GRID_VERTICAL_OFFSET),
                    ),
                    ..default()
                },
                Name::new("SubGridChild"),
            ));

            if let Some(render_layers) = render_layers {
                child_commands.insert(render_layers.clone());
            }
        });
    }
}

/// System for meshing grid axis
pub fn grid_axis_mesher(
    mut commands: Commands,
    query_parent: Query<
        (
            Entity,
            &Grid,
            Option<&GridAxis>,
            Option<&RenderLayers>,
            Option<&Children>,
        ),
        (Or<(Changed<Grid>, Changed<GridAxis>, Changed<RenderLayers>)>,),
    >,
    query_children: Query<Entity, With<GridAxisChild>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut simple_materials: ResMut<Assets<SimpleLineMaterial>>,
) {
    for (entity, grid, axis, render_layers, children) in query_parent.iter() {
        if let Some(children) = children {
            despawn_children_of_type(&mut commands, entity, children, &query_children);
        }

        commands.entity(entity).with_children(|children| {
            let size = grid.count as f32 * grid.spacing;
            let mut common_axis = Vec::<GridAlignment>::new();
            if let Some(axis) = axis {
                let (used, unused) = axis.create_axis();
                common_axis.extend(&unused);
                for (alignment, color) in used {
                    let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::all());
                    mesh.insert_attribute(
                        Mesh::ATTRIBUTE_POSITION,
                        GridAxis::create_single_axis(size, alignment).to_vec(),
                    );
                    let mut commands = children.spawn((
                        GridAxisChild,
                        NotShadowCaster,
                        Name::new("GridAxisChild"),
                        MaterialMesh2dBundle {
                            mesh: Mesh2dHandle(meshes.add(mesh.clone())),
                            material: simple_materials
                                .add(SimpleLineMaterial::from_color(color, grid.alpha_mode)),
                            transform: Transform::from_translation(
                                alignment.shift_vec3(-Vec3::Y * SUB_GRID_VERTICAL_OFFSET),
                            ),
                            ..default()
                        },
                    ));
                    if let Some(render_layers) = render_layers {
                        commands.insert(render_layers.clone());
                    }
                }
            } else {
                common_axis.extend(&GridAxis::default_axis());
            }

            if !common_axis.is_empty() {
                let vertices = common_axis
                    .into_iter()
                    .flat_map(|alignment| GridAxis::create_single_axis(size, alignment))
                    .collect::<Vec<_>>();
                let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::all());
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
                let mut commands = children.spawn((
                    GridAxisChild,
                    MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(mesh.clone())),
                        material: simple_materials
                            .add(SimpleLineMaterial::from_color(grid.color, grid.alpha_mode)),
                        ..default()
                    },
                    Name::new("GridAxisChild"),
                ));
                if let Some(render_layers) = render_layers {
                    commands.insert(render_layers.clone());
                }
            }
        });
    }
}

/// Despawns children with a marker component upon the removal of their parent
pub fn despawn_children_upon_removal<RemovedParent: Component, ChildMarker: Component>(
    mut removed: RemovedComponents<RemovedParent>,
    query: Query<(&Parent, Entity), With<ChildMarker>>,
    mut commands: Commands,
) {
    if removed.is_empty() {
        return;
    }
    let mut parent_to_child_map: HashMap<Entity, Vec<Entity>> = HashMap::new();
    for (parent, child) in query.iter() {
        parent_to_child_map
            .entry(parent.get())
            .and_modify(|children| children.push(child))
            .or_insert_with(|| vec![child]);
    }
    for entity in removed
        .read()
        .filter_map(|entity| parent_to_child_map.get(&entity))
        .flatten()
    {
        commands.entity(*entity).despawn();
    }
}
