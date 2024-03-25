use bevy::{
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
    utils::HashSet,
    window::PrimaryWindow,
};
use hexx::*;
use std::collections::HashMap;

/// World size of the hexagons (outer radius)
const HEX_SIZE: Vec2 = Vec2::splat(15.0);

pub fn main() {
    App::new()
        .init_resource::<HexArea>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_920.0, 1_080.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_grid))
        .add_systems(Update, (handle_input, gizmos).chain())
        .run();
}

#[derive(Debug, Default, Resource)]
struct HexArea {
    pub area: HashSet<Hex>,
}

#[derive(Debug, Resource)]
struct Map {
    flat_layout: HexLayout,
    pointy_layout: HexLayout,
    flat_entities: HashMap<Hex, Entity>,
    pointy_entities: HashMap<Hex, Entity>,
    flat_cursor_entity: Entity,
    pointy_cursor_entity: Entity,
    area_material: Handle<ColorMaterial>,
    default_material: Handle<ColorMaterial>,
}

/// 3D Orthogrpahic camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// Hex grid setup
fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let flat_layout = HexLayout {
        hex_size: HEX_SIZE,
        orientation: HexOrientation::Flat,
        origin: Vec2::new(-480.0, 0.0),
        ..default()
    };
    let pointy_layout = HexLayout {
        hex_size: HEX_SIZE,
        orientation: HexOrientation::Pointy,
        origin: Vec2::new(480.0, 0.0),
        ..default()
    };
    // materials
    let area_material = materials.add(Color::GOLD);
    let default_material = materials.add(Color::WHITE);
    let cursor_material = materials.add(Color::RED);

    // mesh
    let mut spawn_map = |layout: &HexLayout| {
        let mesh_handle = meshes.add(hexagonal_plane(layout));
        let cursor_mesh = meshes.add(border_plane(layout));

        let cursor_entity = commands
            .spawn(ColorMesh2dBundle {
                mesh: cursor_mesh.into(),
                material: cursor_material.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 10.0),
                ..default()
            })
            .id();
        let entities = Hex::ZERO
            .range(15)
            .map(|hex| {
                let pos = layout.hex_to_world_pos(hex);
                let id = commands
                    .spawn(ColorMesh2dBundle {
                        transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                        mesh: mesh_handle.clone().into(),
                        material: default_material.clone(),
                        ..default()
                    })
                    .id();
                (hex, id)
            })
            .collect();
        (cursor_entity, entities)
    };

    let (flat_cursor_entity, flat_entities) = spawn_map(&flat_layout);
    let (pointy_cursor_entity, pointy_entities) = spawn_map(&pointy_layout);
    commands.insert_resource(Map {
        flat_layout,
        pointy_layout,
        flat_entities,
        pointy_entities,
        flat_cursor_entity,
        pointy_cursor_entity,
        area_material,
        default_material,
    });
}

/// Input interaction
fn handle_input(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    map: Res<Map>,
    mut area: ResMut<HexArea>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut cursors: Query<&mut Transform>,
) {
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p))
    else {
        return;
    };
    let mut to_add = Vec::new();
    let mut to_remove = Vec::new();
    for (layout, entities, cursor) in [
        (
            &map.flat_layout,
            &map.flat_entities,
            &map.flat_cursor_entity,
        ),
        (
            &map.pointy_layout,
            &map.pointy_entities,
            &map.pointy_cursor_entity,
        ),
    ] {
        let coord = layout.world_pos_to_hex(pos);
        if entities.get(&coord).is_none() {
            continue;
        };
        let mut cursor = cursors.get_mut(*cursor).unwrap();
        let pos = layout.hex_to_world_pos(coord);
        cursor.translation.x = pos.x;
        cursor.translation.y = pos.y;
        if mouse.pressed(MouseButton::Left) {
            to_add.push(coord);
        } else if mouse.pressed(MouseButton::Right) {
            to_remove.push(coord);
        }
    }
    for coord in to_add {
        area.area.insert(coord);
        let entity = map.flat_entities.get(&coord).unwrap();
        commands.entity(*entity).insert(map.area_material.clone());
        let entity = map.pointy_entities.get(&coord).unwrap();
        commands.entity(*entity).insert(map.area_material.clone());
    }
    for coord in to_remove {
        area.area.remove(&coord);
        let entity = map.flat_entities.get(&coord).unwrap();
        commands
            .entity(*entity)
            .insert(map.default_material.clone());
        let entity = map.pointy_entities.get(&coord).unwrap();
        commands
            .entity(*entity)
            .insert(map.default_material.clone());
    }
}

fn gizmos(mut gizmos: Gizmos, area: Res<HexArea>, map: Res<Map>) {
    let mut edges = Vec::new();
    for hex in &area.area {
        for neighbor in hex
            .all_neighbors()
            .iter()
            .filter(|c| !area.area.contains(*c))
        {
            edges.push(GridEdge {
                origin: *hex,
                direction: hex.neighbor_direction(*neighbor).unwrap(),
            });
        }
    }
    for layout in [&map.flat_layout, &map.pointy_layout] {
        for edge in &edges {
            let [a, b] = layout.edge_coordinates(*edge);
            gizmos.line_2d(a, b, Color::ORANGE);
        }
    }
}

/// Compute a bevy mesh from the layout
fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(Vec3::Z)
        .center_aligned()
        .build();
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}

fn border_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(Vec3::Z)
        .with_inset_options(InsetOptions {
            keep_inner_face: false,
            scale: 0.2,
            ..default()
        })
        .center_aligned()
        .build();
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}
