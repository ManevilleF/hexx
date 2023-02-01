use std::collections::HashMap;

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use hexx::shapes;
use hexx::*;

const HEX_SIZE: Vec2 = Vec2::splat(15.0);

pub fn main() {
    App::new()
        .insert_resource(AmbientLight {
            brightness: 1.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_grid)
        .add_system(handle_input)
        .run();
}

#[derive(Debug, Default, Resource)]
struct SelectedHex(Hex);

#[derive(Debug, Default, Resource)]
struct HighlightedHexes(Vec<Hex>);

#[derive(Debug, Resource)]
struct Map {
    layout: HexLayout,
    entities: HashMap<Hex, Entity>,
    selected_material: Handle<StandardMaterial>,
    highlighted_material: Handle<StandardMaterial>,
    default_material: Handle<StandardMaterial>,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, -Vec3::Z),
        projection: OrthographicProjection::default().into(),
        ..default()
    });
}

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
    let highlighted_material = materials.add(Color::YELLOW.into());
    let default_material = materials.add(Color::WHITE.into());
    // mesh
    let mesh = hexagonal_plane(&layout);
    let mesh = meshes.add(mesh);

    let entities = shapes::hexagon(Hex::ZERO, 30)
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let id = commands
                .spawn(PbrBundle {
                    transform: Transform::from_xyz(pos.x, 0.0, pos.y).with_scale(Vec3::splat(0.9)),
                    mesh: mesh.clone(),
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
        highlighted_material,
        default_material,
    });
}

fn handle_input(
    mut commands: Commands,
    windows: Res<Windows>,
    map: Res<Map>,
    mut selected_hex: Local<SelectedHex>,
    mut highlighted_hexes: Local<HighlightedHexes>,
) {
    let window = windows.primary();
    if let Some(pos) = window.cursor_position() {
        let pos = Vec2::new(pos.x, window.height() - pos.y)
            - Vec2::new(window.width(), window.height()) / 2.0;
        let hex = map.layout.world_pos_to_hex(pos);
        if let Some(entity) = map.entities.get(&hex).copied() {
            if hex == selected_hex.0 {
                return;
            }
            // Clear highlighted hexes materials
            for entity in highlighted_hexes
                .0
                .iter()
                .filter_map(|h| map.entities.get(h))
            {
                commands
                    .entity(*entity)
                    .insert(map.default_material.clone());
            }
            commands
                .entity(map.entities[&selected_hex.0])
                .insert(map.default_material.clone());
            // Draw a  line
            highlighted_hexes.0 = Hex::ZERO.line_to(hex).collect();
            // Draw a ring
            highlighted_hexes.0.extend(Hex::ZERO.ring(hex.ulength()));
            for h in &highlighted_hexes.0 {
                if let Some(e) = map.entities.get(h) {
                    commands.entity(*e).insert(map.highlighted_material.clone());
                }
            }
            // Make the selected tile red
            commands
                .entity(entity)
                .insert(map.selected_material.clone());
            selected_hex.0 = hex;
        }
    }
}

fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = MeshInfo::hexagonal_plane(hex_layout, Hex::ZERO);
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs.to_vec());
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
}
