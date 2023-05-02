use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    utils::{HashMap, HashSet},
    window::PrimaryWindow,
};
use hexx::{algorithms::range_fov, *};

/// World size of the hexagons (outer radius)
const HEX_SIZE: Vec2 = Vec2::splat(14.0);
const MAP_RADIUS: u32 = 20;
const FOV_RADIUS: u32 = 100;

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
    pub visible_entities: HashSet<Entity>,
    pub layout: HexLayout,
    pub default_mat: Handle<ColorMaterial>,
    pub blocked_mat: Handle<ColorMaterial>,
    pub visible_mat: Handle<ColorMaterial>,
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
    let visible_mat = materials.add(Color::CYAN.into());
    let mut blocked_coords = HashSet::new();
    let entities = Hex::ZERO
        .spiral_range(0..=MAP_RADIUS)
        .enumerate()
        .map(|(i, coord)| {
            let pos = layout.hex_to_world_pos(coord);
            let material = if i % 10 == 0 {
                blocked_coords.insert(coord);
                blocked_mat.clone()
            } else {
                default_mat.clone()
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
        visible_entities: Default::default(),
        layout,
        default_mat,
        blocked_mat,
        visible_mat,
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
            if grid.blocked_coords.contains(&hex_pos) {
                grid.blocked_coords.remove(&hex_pos);
                commands.entity(entity).insert(grid.default_mat.clone());
            } else {
                grid.blocked_coords.insert(hex_pos);
                grid.visible_entities.remove(&entity);
                commands.entity(entity).insert(grid.blocked_mat.clone());
            }
            return;
        }
        if hex_pos == *current {
            return;
        }
        *current = hex_pos;
        for entity in &grid.visible_entities {
            commands.entity(*entity).insert(grid.default_mat.clone());
        }
        let fov = range_fov(hex_pos, FOV_RADIUS, |h| {
            grid.blocked_coords.contains(&h) || h.ulength() > MAP_RADIUS
        });
        let entities: HashSet<_> = fov
            .into_iter()
            .filter_map(|h| grid.entities.get(&h).copied())
            .collect();
        for entity in &entities {
            commands.entity(*entity).insert(grid.visible_mat.clone());
        }
        grid.visible_entities = entities;
    }
}

/// Compute a bevy mesh from the layout
fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout).facing(Vec3::Z).build();
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs);
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
}
