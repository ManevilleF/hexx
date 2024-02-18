use bevy::{
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
    time::common_conditions::on_timer,
};
use hexx::{shapes, *};
use std::{collections::HashMap, time::Duration};

/// World size of the hexagons (outer radius)
const HEX_SIZE: Vec2 = Vec2::splat(1.0);
/// World space height of hex columns
const COLUMN_HEIGHT: f32 = 10.0;
/// Map radius
const MAP_RADIUS: u32 = 20;
/// Animation time step
const TIME_STEP: Duration = Duration::from_millis(100);

pub fn main() {
    App::new()
        .insert_resource(AmbientLight {
            brightness: 0.1,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_grid))
        .add_systems(Update, animate_rings.run_if(on_timer(TIME_STEP)))
        .run();
}

#[derive(Debug, Resource)]
struct Map {
    entities: HashMap<Hex, Entity>,
    highlighted_material: Handle<StandardMaterial>,
    default_material: Handle<StandardMaterial>,
}

#[derive(Debug, Default, Resource)]
struct HighlightedHexes {
    ring: u32,
    hexes: Vec<Hex>,
}

/// 3D Orthogrpahic camera setup
fn setup_camera(mut commands: Commands) {
    let transform = Transform::from_xyz(0.0, 60.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn(Camera3dBundle {
        transform,
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        transform,
        ..default()
    });
}

/// Hex grid setup
fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let layout = HexLayout {
        hex_size: HEX_SIZE,
        ..default()
    };
    // materials
    let default_material = materials.add(Color::WHITE);
    let highlighted_material = materials.add(Color::YELLOW);
    // mesh
    let mesh = hexagonal_column(&layout);
    let mesh_handle = meshes.add(mesh);

    let entities = shapes::hexagon(Hex::ZERO, MAP_RADIUS)
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let id = commands
                .spawn(PbrBundle {
                    transform: Transform::from_xyz(pos.x, hex.length() as f32 / 2.0, pos.y),
                    mesh: mesh_handle.clone(),
                    material: default_material.clone(),
                    ..default()
                })
                .id();
            (hex, id)
        })
        .collect();
    commands.insert_resource(Map {
        entities,
        highlighted_material,
        default_material,
    });
}

fn animate_rings(
    mut commands: Commands,
    map: Res<Map>,
    mut highlighted_hexes: Local<HighlightedHexes>,
) {
    // Clear highlighted hexes materials
    for entity in highlighted_hexes
        .hexes
        .iter()
        .filter_map(|h| map.entities.get(h))
    {
        commands
            .entity(*entity)
            .insert(map.default_material.clone());
    }
    highlighted_hexes.ring += 1;
    if highlighted_hexes.ring > MAP_RADIUS {
        highlighted_hexes.ring = 0;
    }
    highlighted_hexes.hexes = Hex::ZERO.ring(highlighted_hexes.ring).collect();
    // Draw a ring
    for h in &highlighted_hexes.hexes {
        if let Some(e) = map.entities.get(h) {
            commands.entity(*e).insert(map.highlighted_material.clone());
        }
    }
}

/// Compute a bevy mesh from the layout
fn hexagonal_column(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = ColumnMeshBuilder::new(hex_layout, COLUMN_HEIGHT)
        .without_bottom_face()
        .with_scale(Vec3::splat(0.9))
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
