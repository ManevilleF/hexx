use std::collections::HashMap;

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use hexx::shapes;
use hexx::*;

/// World size of the hexagons (outer radius)
const HEX_SIZE: Vec2 = Vec2::splat(8.0);

pub fn main() {
    App::new()
        .insert_resource(AmbientLight {
            brightness: 1.0,
            ..default()
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1_000.0,
                height: 1_000.0,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup_camera)
        .add_startup_system(setup_grid)
        .add_system(handle_input)
        .run();
}

#[derive(Debug, Default, Resource)]
struct HighlightedHexes {
    pub selected: Hex,
    pub halfway: Hex,
    pub ring: Vec<Hex>,
    pub edge: Vec<Hex>,
    pub line: Vec<Hex>,
    pub half_ring: Vec<Hex>,
    pub rotated: Vec<Hex>,
}

#[derive(Debug, Resource)]
struct Map {
    layout: HexLayout,
    entities: HashMap<Hex, Entity>,
    selected_material: Handle<StandardMaterial>,
    ring_material: Handle<StandardMaterial>,
    edge_material: Handle<StandardMaterial>,
    line_material: Handle<StandardMaterial>,
    half_ring_material: Handle<StandardMaterial>,
    default_material: Handle<StandardMaterial>,
}

/// 3D Orthogrpahic camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, -Vec3::Z),
        projection: OrthographicProjection::default().into(),
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
    let selected_material = materials.add(Color::RED.into());
    let ring_material = materials.add(Color::YELLOW.into());
    let edge_material = materials.add(Color::CYAN.into());
    let line_material = materials.add(Color::ORANGE.into());
    let half_ring_material = materials.add(Color::LIME_GREEN.into());
    let default_material = materials.add(Color::WHITE.into());
    // mesh
    let mesh = hexagonal_plane(&layout);
    let mesh_handle = meshes.add(mesh);

    let entities = shapes::flat_rectangle([-40, 40, -35, 35])
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
        layout,
        entities,
        selected_material,
        ring_material,
        default_material,
        line_material,
        half_ring_material,
        edge_material,
    });
}

/// Input interaction
fn handle_input(
    mut commands: Commands,
    windows: Res<Windows>,
    map: Res<Map>,
    mut highlighted_hexes: Local<HighlightedHexes>,
) {
    let window = windows.primary();
    if let Some(pos) = window.cursor_position() {
        let pos = Vec2::new(pos.x, window.height() - pos.y)
            - Vec2::new(window.width(), window.height()) / 2.0;
        let hex = map.layout.world_pos_to_hex(pos);
        if let Some(entity) = map.entities.get(&hex).copied() {
            if hex == highlighted_hexes.selected {
                return;
            }
            // Clear highlighted hexes materials
            for vec in [
                &highlighted_hexes.ring,
                &highlighted_hexes.line,
                &highlighted_hexes.edge,
                &highlighted_hexes.half_ring,
                &highlighted_hexes.rotated,
            ] {
                for entity in vec.iter().filter_map(|h| map.entities.get(h)) {
                    commands
                        .entity(*entity)
                        .insert(map.default_material.clone());
                }
            }
            commands
                .entity(map.entities[&highlighted_hexes.selected])
                .insert(map.default_material.clone());
            commands
                .entity(map.entities[&highlighted_hexes.halfway])
                .insert(map.default_material.clone());
            // Draw a  line
            highlighted_hexes.line = Hex::ZERO.line_to(hex).collect();
            // Draw a ring
            highlighted_hexes.ring = Hex::ZERO.ring(hex.ulength());
            // Draw an edge
            highlighted_hexes.edge = Hex::ZERO.ring_edge(hex.ulength(), Default::default());
            // Draw a half ring
            highlighted_hexes.half_ring = Hex::ZERO.ring(hex.ulength() / 2);
            // Draw rotations
            highlighted_hexes.rotated = (1..6).map(|i| hex.rotate_right(i)).collect();
            for (vec, mat) in [
                (&highlighted_hexes.ring, &map.ring_material),
                (&highlighted_hexes.edge, &map.edge_material),
                (&highlighted_hexes.line, &map.line_material),
                (&highlighted_hexes.half_ring, &map.half_ring_material),
                (&highlighted_hexes.rotated, &map.selected_material),
            ] {
                for h in vec {
                    if let Some(e) = map.entities.get(h) {
                        commands.entity(*e).insert(mat.clone());
                    }
                }
            }
            // Make the half selction red
            highlighted_hexes.halfway = hex / 2;
            commands
                .entity(map.entities[&highlighted_hexes.halfway])
                .insert(map.selected_material.clone());
            // Make the selected tile red
            commands
                .entity(entity)
                .insert(map.selected_material.clone());
            highlighted_hexes.selected = hex;
        }
    }
}

/// Compute a bevy mesh from the layout
fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = MeshInfo::hexagonal_plane(hex_layout, Hex::ZERO);
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs.to_vec());
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
}
