use super::{collapsible_header_config, entity_entry_config, PerfUiEntry};
use crate::{prelude::*, utils};
use bevy::{
    diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin},
    ecs::{archetype::Archetype, system::SystemState, world::unsafe_world_cell::UnsafeWorldCell},
    prelude::*,
    utils::{info, HashMap, HashSet},
};
use reactor_ui::{prelude::*, sickle::prelude::*};

#[derive(Component, Debug, Clone, Default, Reflect)]
pub struct HierarchyUiEntryEntityList;

#[derive(Component, Debug, Clone, Reflect)]
pub struct HierarchyUiEntryEntityListContainer {
    pub displayed_entity: Entity,
}

#[derive(Default, Component, Debug, Clone, Reflect)]
pub struct HierarchyUiEntryEntityListRootContainer;

#[derive(Default, Component, Debug, Clone, Reflect)]
pub struct HierarchyUiEntryEntityDisplay;

#[derive(Default, Component, Debug, Clone, Reflect)]
pub struct IgnoreInHierarchy;

impl PerfUiEntry for HierarchyUiEntryEntityList {
    fn setup(app: &mut App) {
        app.add_systems(
            Update,
            (add_to_ignore_list, add_entities, build_entry)
                .chain()
                .in_set(ReactorPerfUiSchedule::Update),
        )
        .add_systems(
            Update,
            (remove_from_ignore_list, parent_changed, entity_despawned)
                .in_set(ReactorPerfUiSchedule::Update),
        );

        app.add_event::<BuildHierarchyNode>();

        app.register_type::<HierarchyUiEntryEntityList>()
            .register_type::<HierarchyUiEntryEntityListContainer>()
            .register_type::<HierarchyUiEntryEntityListRootContainer>()
            .register_type::<HierarchyUiEntryEntityDisplay>();

        app.init_resource::<HierarchyParentContainerLookup>()
            .init_resource::<HierarchySelfContainerLookup>()
            .init_resource::<HierarchyRootNode>()
            .init_resource::<HierarchyIgnoredNodes>();
    }

    fn spawn(list: &mut UiBuilder<Entity>) {
        list.list_item_collapsible_header(
            collapsible_header_config("Entities".into()),
            |container| {
                container
                    .insert(HierarchyUiEntryEntityListRootContainer)
                    .insert(IgnoreInHierarchy);
                let id = container.id();
                let commands = container.commands();
                commands.insert_resource(HierarchyRootNode(Some(id)));
            },
        );

        list.insert(HierarchyUiEntryEntityList);
    }
}

type Container = Entity;

#[derive(Default, Resource, Debug, Clone, Deref, DerefMut)]
pub struct HierarchyParentContainerLookup(pub HashMap<Entity, Container>);

#[derive(Default, Resource, Debug, Clone, Deref, DerefMut)]
pub struct HierarchySelfContainerLookup(pub HashMap<Entity, Container>);

#[derive(Default, Resource, Debug, Clone, Deref, DerefMut)]
pub struct HierarchyRootNode(pub Option<Container>);

#[derive(Default, Resource, Debug, Clone, Deref, DerefMut)]
pub struct HierarchyIgnoredNodes(pub HashSet<Container>);

#[derive(Event, Debug, Clone)]
pub struct BuildHierarchyNode {
    pub container: Container,
    pub target: Entity,
}

#[derive(Default, Component, Debug, Clone, Reflect)]
pub struct InHierarchyDisplay;

fn add_to_ignore_list(
    mut commands: Commands,
    to_ignore: Query<Entity, Added<IgnoreInHierarchy>>,
    mut ignore_list: ResMut<HierarchyIgnoredNodes>,
    mut self_container_lookup: ResMut<HierarchySelfContainerLookup>,
) {
    for e in &to_ignore {
        ignore_list.insert(e);

        if let Some(self_container) = self_container_lookup.get(&e) {
            commands.entity(*self_container).despawn_recursive();
            self_container_lookup.remove(&e);
        }
    }
}

fn parent_changed(
    mut commands: Commands,
    changed: Query<Entity, (Changed<Parent>, With<InHierarchyDisplay>)>,
    mut self_container_lookup: ResMut<HierarchySelfContainerLookup>,
) {
    for e in &changed {
        // Remove your existing entry
        if let Some(self_container) = self_container_lookup.get(&e) {
            commands.entity(*self_container).despawn_recursive();
            self_container_lookup.remove(&e);
        }
        // Mark you as no longer in the hierarchy (you'll get rebuilt on next frame)
        commands.entity(e).remove::<InHierarchyDisplay>();
    }
}

fn entity_despawned(
    mut commands: Commands,
    mut self_container_lookup: ResMut<HierarchySelfContainerLookup>,
) {
    let mut to_remove = Vec::new();
    for (e, c) in self_container_lookup.iter() {
        let status = commands.get_entity(*e);
        if status.is_none() {
            commands.entity(*c).despawn_recursive();
            to_remove.push(*e);
        }
    }

    self_container_lookup.retain(|e, _| !to_remove.contains(e));
}

fn remove_from_ignore_list(
    mut to_remove: RemovedComponents<IgnoreInHierarchy>,
    mut ignore_list: ResMut<HierarchyIgnoredNodes>,
) {
    for e in to_remove.read() {
        ignore_list.remove(&e);
    }
}

fn add_entities(
    mut commands: Commands,
    all_entities: Query<
        (Entity, Option<&Parent>),
        (Without<IgnoreInHierarchy>, Without<InHierarchyDisplay>),
    >,
    mut events: EventWriter<BuildHierarchyNode>,
    parent_container_lookup: ResMut<HierarchyParentContainerLookup>,
    root_node: Res<HierarchyRootNode>,
    ignore_list: ResMut<HierarchyIgnoredNodes>,
) {
    if root_node.is_none() {
        return;
    }

    if ignore_list.is_empty() {
        return;
    }

    for (e, parent) in &all_entities {
        if ignore_list.contains(&e) {
            continue;
        }

        if let Some(parent) = parent {
            if ignore_list.contains(&parent.get()) {
                commands.entity(e).insert(IgnoreInHierarchy);
                continue;
            }
        }

        let mut parent_container = None;
        match parent {
            None => {
                parent_container = root_node.0;
            }
            Some(p) => {
                let p = p.get();
                if let Some(c) = parent_container_lookup.get(&p) {
                    parent_container = Some(*c);
                }
            }
        }

        if let Some(c) = parent_container {
            commands.entity(e).insert(InHierarchyDisplay);
            events.send(BuildHierarchyNode {
                container: c,
                target: e,
            });
        }
    }
}

fn build_entry(
    world: &mut World,
    params: &mut SystemState<(
        EventReader<BuildHierarchyNode>,
        ResMut<HierarchySelfContainerLookup>,
        ResMut<HierarchyParentContainerLookup>,
    )>,
) {
    let mut to_build: HashMap<Container, Vec<Entity>> = HashMap::new();
    {
        let (mut events, _, _) = params.get_mut(world);

        for event in events.read() {
            to_build
                .entry(event.container)
                .or_default()
                .push(event.target);
        }
    }

    let mut to_add_to_lookup = Vec::new();
    for (container, targets) in to_build {
        for e in targets {
            to_add_to_lookup.push(build_node(world, container, e));
        }
    }

    {
        let (_, mut self_container_lookup, mut parent_container_lookup) = params.get_mut(world);

        for (e, s, c) in to_add_to_lookup {
            parent_container_lookup.insert(e, c);
            self_container_lookup.insert(e, s);
        }
    }
}

type ParentContainer = Container;
type ChildContainer = Container;
type SelfContainer = Container;

fn build_node(
    world: &mut World,
    parent_container: ParentContainer,
    target: Entity,
) -> (Entity, SelfContainer, ChildContainer) {
    // info!("Building {} in {}", target, parent_container);
    let name = get_name(target, world);
    let mut commands = world.commands();
    let mut builder = commands.ui_builder(parent_container);
    let (self_container, child_container) = builder.entity_entry(target, name);

    (target, self_container, child_container)
}

fn get_name(entity: Entity, world: &World) -> String {
    match world.get_entity(entity) {
        Some(entity_ref) => {
            if let Some(name) = entity_ref.get::<Name>() {
                return format!("{} ({})", name.as_str(), entity);
            }

            get_name_inner(
                world.as_unsafe_world_cell_readonly(),
                entity,
                entity_ref.archetype(),
            )
        }
        None => format!("Entity {} (dead)", entity.index()),
    }
}

fn get_name_inner(world: UnsafeWorldCell<'_>, entity: Entity, archetype: &Archetype) -> String {
    #[rustfmt::skip]
    let associations = &[
        ("bevy_window::window::PrimaryWindow", "Primary Window"),
        ("bevy_core_pipeline::core_3d::camera_3d::Camera3d", "Camera3d"),
        ("bevy_core_pipeline::core_2d::camera_2d::Camera2d", "Camera2d"),
        ("bevy_pbr::light::PointLight", "PointLight"),
        ("bevy_pbr::light::DirectionalLight", "DirectionalLight"),
        ("bevy_text::text::Text", "Text"),
        ("bevy_ui::ui_node::Node", "Node"),
        ("bevy_asset::handle::Handle<bevy_pbr::pbr_material::StandardMaterial>", "Pbr Mesh"),
        ("bevy_window::window::Window", "Window"),
    ];

    let type_names = archetype.components().filter_map(|id| {
        let name = world.components().get_info(id)?.name();
        Some(name)
    });

    for component_type in type_names {
        if let Some(name) = associations
            .iter()
            .find_map(|&(name, matches)| (component_type == name).then_some(matches))
        {
            return format!("{name} ({entity})");
        }
    }

    format!("Entity ({entity})")
}
