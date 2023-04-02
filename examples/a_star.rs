use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    utils::{HashMap, HashSet},
    window::PrimaryWindow,
};
use hexx::{algorithms::a_star, *};

/// World size of the hexagons (outer radius)
const HEX_SIZE: Vec2 = Vec2::splat(10.0);
const MAP_RADIUS: u32 = 30;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_000.0, 1_000.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup_camera)
        .add_startup_system(setup_grid)
        .add_system(handle_input)
        .run();
}

#[derive(Debug, Resource)]
struct HexGrid {
    pub entities: HashMap<Hex, Entity>,
    pub blocked_coords: HashSet<Hex>,
    pub path_entities: HashSet<Entity>,
    pub layout: HexLayout,
    pub default_mat: Handle<ColorMaterial>,
    pub blocked_mat: Handle<ColorMaterial>,
    pub path_mat: Handle<ColorMaterial>,
}

/// 3D Orthogrpahic camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let layout = HexLayout {
        hex_size: HEX_SIZE,
        ..default()
    };
    let mesh = meshes.add(hexagonal_plane(&layout));
    let default_mat = materials.add(Color::WHITE.into());
    let blocked_mat = materials.add(Color::BLACK.into());
    let path_mat = materials.add(Color::CYAN.into());
    let mut blocked_coords = HashSet::new();
    let entities = Hex::ZERO
        .spiral_range(0..=MAP_RADIUS)
        .enumerate()
        .map(|(i, coord)| {
            let pos = layout.hex_to_world_pos(coord);
            let material = match coord {
                c if i % 5 == 0 => {
                    blocked_coords.insert(c);
                    blocked_mat.clone()
                }
                _ => default_mat.clone(),
            };
            let entity = commands
                .spawn(ColorMesh2dBundle {
                    mesh: mesh.clone().into(),
                    material,
                    transform: Transform::from_xyz(pos.x, pos.y, 0.0).with_scale(Vec3::splat(0.9)),
                    ..default()
                })
                .id();
            (coord, entity)
        })
        .collect();
    commands.insert_resource(HexGrid {
        entities,
        blocked_coords,
        path_entities: Default::default(),
        layout,
        default_mat,
        blocked_mat,
        path_mat,
    })
}

/// Input interaction
fn handle_input(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut current: Local<Hex>,
    mut grid: ResMut<HexGrid>,
) {
    let window = windows.single();
    if let Some(pos) = window.cursor_position() {
        let pos = pos - Vec2::new(window.width(), window.height()) / 2.0;
        let hex_pos = grid.layout.world_pos_to_hex(pos);
        let Some(entity) = grid.entities.get(&hex_pos).copied() else { return };
        if buttons.just_pressed(MouseButton::Left) {
            //
            if grid.blocked_coords.contains(&hex_pos) {
                grid.blocked_coords.remove(&hex_pos);
                commands.entity(entity).insert(grid.default_mat.clone());
            } else {
                grid.blocked_coords.insert(hex_pos);
                grid.path_entities.remove(&entity);
                commands.entity(entity).insert(grid.blocked_mat.clone());
            }
            return;
        }
        if hex_pos == *current {
            return;
        }
        *current = hex_pos;
        for entity in &grid.path_entities {
            commands.entity(*entity).insert(grid.default_mat.clone());
        }
        let path = a_star(Hex::ZERO, hex_pos, |h| {
            if grid.blocked_coords.contains(&h) || h.ulength() > MAP_RADIUS {
                None
            } else {
                Some(0)
            }
        });
        let entities: HashSet<_> = path
            .into_iter()
            .filter_map(|h| grid.entities.get(&h).copied())
            .collect();
        for entity in &entities {
            commands.entity(*entity).insert(grid.path_mat.clone());
        }
        grid.path_entities = entities;
    }
}

/// Compute a bevy mesh from the layout
fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = MeshInfo::hexagonal_plane(hex_layout, Hex::ZERO).facing(Vec3::Z);
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs.to_vec());
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
}
