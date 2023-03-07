use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::time::common_conditions::on_fixed_timer;
use hexx::shapes;
use hexx::*;
use std::collections::HashMap;
use std::time::Duration;

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
            brightness: 1.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_grid)
        .add_system(
            animate_rings
                .in_schedule(CoreSchedule::FixedUpdate)
                .run_if(on_fixed_timer(TIME_STEP)),
        )
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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
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
    let default_material = materials.add(Color::WHITE.into());
    let highlighted_material = materials.add(Color::YELLOW.into());
    // mesh
    let mesh = hexagonal_column(&layout);
    let mesh_handle = meshes.add(mesh);

    let entities = shapes::hexagon(Hex::ZERO, MAP_RADIUS)
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let id = commands
                .spawn(PbrBundle {
                    transform: Transform::from_xyz(pos.x, 0.0, pos.y).with_scale(Vec3::splat(0.9)),
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
    highlighted_hexes.hexes = Hex::ZERO.ring(highlighted_hexes.ring);
    // Draw a ring
    for h in &highlighted_hexes.hexes {
        if let Some(e) = map.entities.get(h) {
            commands.entity(*e).insert(map.highlighted_material.clone());
        }
    }
}

/// Compute a bevy mesh from the layout
fn hexagonal_column(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = MeshInfo::partial_hexagonal_column(hex_layout, Hex::ZERO, COLUMN_HEIGHT);
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs.to_vec());
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
}
