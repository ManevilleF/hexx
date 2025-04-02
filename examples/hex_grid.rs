use std::collections::HashMap;

use bevy::{
    color::palettes::css::{AQUA, LIMEGREEN, ORANGE, RED, VIOLET, WHITE, YELLOW},
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
    window::PrimaryWindow,
};
use hexx::{shapes, *};

/// World size of the hexagons (outer radius)
const HEX_SIZE: f32 = 13.0;

pub fn main() {
    App::new()
        .init_resource::<HighlightedHexes>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_000.0, 1_000.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_grid))
        .add_systems(Update, (handle_input, gizmos).chain())
        .run();
}

#[derive(Debug, Default, Resource)]
struct HighlightedHexes {
    pub selected: Hex,
    pub halfway: Hex,
    pub ring: Vec<Hex>,
    pub wedge: Vec<Hex>,
    pub dir_wedge: Vec<Hex>,
    pub line: Vec<Hex>,
    pub rectiline: Vec<Hex>,
    pub half_ring: Vec<Hex>,
    pub rotated: Vec<Hex>,
}

#[derive(Debug, Resource)]
struct Map {
    layout: HexLayout,
    entities: HashMap<Hex, Entity>,
    selected_material: Handle<ColorMaterial>,
    ring_material: Handle<ColorMaterial>,
    wedge_material: Handle<ColorMaterial>,
    dir_wedge_material: Handle<ColorMaterial>,
    line_material: Handle<ColorMaterial>,
    half_ring_material: Handle<ColorMaterial>,
    default_material: Handle<ColorMaterial>,
}

/// 2D camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Hex grid setup
fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let layout = HexLayout::flat().with_hex_size(HEX_SIZE);
    // materials
    let selected_material = materials.add(Color::Srgba(RED));
    let ring_material = materials.add(Color::Srgba(YELLOW));
    let wedge_material = materials.add(Color::Srgba(AQUA));
    let dir_wedge_material = materials.add(Color::Srgba(VIOLET));
    let line_material = materials.add(Color::Srgba(ORANGE));
    let half_ring_material = materials.add(Color::Srgba(LIMEGREEN));
    let default_material = materials.add(Color::Srgba(WHITE));
    // mesh
    let mesh = hexagonal_plane(&layout);
    let mesh_handle = meshes.add(mesh);

    let entities = shapes::flat_rectangle([-26, 26, -23, 23])
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let id = commands
                .spawn((
                    Mesh2d(mesh_handle.clone()),
                    MeshMaterial2d(default_material.clone_weak()),
                    Transform::from_xyz(pos.x, pos.y, 0.0),
                    children![(
                        Text2d(format!("{},{}", hex.x, hex.y)),
                        TextColor(Color::BLACK),
                        TextFont {
                            font_size: 7.0,
                            ..default()
                        },
                        Transform::from_xyz(0.0, 0.0, 10.0),
                    )],
                ))
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
        wedge_material,
        dir_wedge_material,
    });
}

/// Input interaction
fn handle_input(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    map: Res<Map>,
    mut highlighted_hexes: ResMut<HighlightedHexes>,
) -> Result {
    let window = windows.single()?;
    let (camera, cam_transform) = cameras.single()?;
    if let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p).ok())
    {
        let coord = map.layout.world_pos_to_hex(pos);
        if let Some(entity) = map.entities.get(&coord).copied() {
            if coord == highlighted_hexes.selected {
                return Ok(());
            }
            // Clear highlighted hexes materials
            for vec in [
                &highlighted_hexes.ring,
                &highlighted_hexes.line,
                &highlighted_hexes.wedge,
                &highlighted_hexes.dir_wedge,
                &highlighted_hexes.half_ring,
                &highlighted_hexes.rotated,
            ] {
                for entity in vec.iter().filter_map(|h| map.entities.get(h)) {
                    commands
                        .entity(*entity)
                        .insert(MeshMaterial2d(map.default_material.clone_weak()));
                }
            }
            commands
                .entity(map.entities[&highlighted_hexes.selected])
                .insert(MeshMaterial2d(map.default_material.clone_weak()));
            commands
                .entity(map.entities[&highlighted_hexes.halfway])
                .insert(MeshMaterial2d(map.default_material.clone_weak()));
            // Draw a line
            highlighted_hexes.line = Hex::ZERO.line_to(coord).collect();
            // Draw a rectiline path
            highlighted_hexes.rectiline = Hex::ZERO.rectiline_to(coord, false).collect();
            // Draw a ring
            highlighted_hexes.ring = Hex::ZERO.ring(coord.ulength()).collect();
            // Draw an wedge
            highlighted_hexes.wedge = Hex::ZERO.wedge_to(coord).collect();
            // Draw a half ring
            highlighted_hexes.half_ring = Hex::ZERO.ring(coord.ulength() / 2).collect();
            // Draw rotations
            highlighted_hexes.rotated = (1..6).map(|i| coord.rotate_cw(i)).collect();
            // Draw an dual wedge
            highlighted_hexes.dir_wedge = Hex::ZERO.corner_wedge_to(coord / 2).collect();
            for (vec, mat) in [
                (&highlighted_hexes.wedge, &map.wedge_material),
                (&highlighted_hexes.dir_wedge, &map.dir_wedge_material),
                (&highlighted_hexes.ring, &map.ring_material),
                (&highlighted_hexes.line, &map.line_material),
                (&highlighted_hexes.half_ring, &map.half_ring_material),
                (&highlighted_hexes.rotated, &map.selected_material),
            ] {
                for h in vec {
                    if let Some(e) = map.entities.get(h) {
                        commands.entity(*e).insert(MeshMaterial2d(mat.clone_weak()));
                    }
                }
            }
            // Make the half selction red
            highlighted_hexes.halfway = coord / 2;
            commands
                .entity(map.entities[&highlighted_hexes.halfway])
                .insert(MeshMaterial2d(map.selected_material.clone_weak()));
            // Make the selected tile red
            commands
                .entity(entity)
                .insert(MeshMaterial2d(map.selected_material.clone_weak()));
            highlighted_hexes.selected = coord;
        }
    }
    Ok(())
}

fn gizmos(mut gizmos: Gizmos, higlights: Res<HighlightedHexes>, map: Res<Map>) {
    for coord in &higlights.rectiline {
        for [a, b] in map.layout.all_edge_coordinates(*coord) {
            gizmos.line_2d(a, b, Color::WHITE);
        }
    }
}

/// Compute a bevy mesh from the layout
fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(Vec3::Z)
        .with_scale(Vec3::splat(0.98))
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
