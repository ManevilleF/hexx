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
    selected_material: Handle<ColorMaterial>,
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
    let selected_material = materials.add(Color::RED);
    let default_material = materials.add(Color::WHITE);
    // mesh
    let mut spawn_map = |layout: &HexLayout| {
        let mesh = hexagonal_plane(layout);
        let mesh_handle = meshes.add(mesh);

        Hex::ZERO
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
            .collect()
    };

    let flat_entities = spawn_map(&flat_layout);
    let pointy_entities = spawn_map(&pointy_layout);
    commands.insert_resource(Map {
        flat_layout,
        pointy_layout,
        flat_entities,
        pointy_entities,
        selected_material,
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
    mut selected: Local<[Hex; 2]>,
) {
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p))
    else {
        return;
    };
    for (layout, entities, selected_idx) in [
        (&map.flat_layout, &map.flat_entities, 0),
        (&map.pointy_layout, &map.pointy_entities, 1),
    ] {
        let coord = layout.world_pos_to_hex(pos);
        let Some(entity) = entities.get(&coord).copied() else {
            continue;
        };
        if coord != selected[selected_idx] {
            let selected_entity = entities.get(&selected[selected_idx]).unwrap();
            commands
                .entity(*selected_entity)
                .insert(map.default_material.clone());
            commands
                .entity(entity)
                .insert(map.selected_material.clone());
            selected[selected_idx] = coord;
        }
        if mouse.pressed(MouseButton::Left) {
            area.area.insert(coord);
        } else if mouse.pressed(MouseButton::Right) {
            area.area.remove(&coord);
        }
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
