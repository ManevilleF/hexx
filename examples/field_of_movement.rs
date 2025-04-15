use bevy::{
    color::palettes::css::{GRAY, LIME, WHITE, YELLOW},
    platform::collections::hash_set::HashSet,
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
    window::PrimaryWindow,
};
use hexx::{
    algorithms::field_of_movement,
    storage::{HexStore, HexagonalMap},
    *,
};
use rand::prelude::*;

/// World size of the hexagons (outer radius)
const HEX_SIZE: Vec2 = Vec2::splat(14.0);
const MAP_RADIUS: u32 = 20;
const BUDGET: u32 = 13;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_000.0, 1_000.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_grid))
        .add_systems(Update, handle_input)
        .run();
}

type Cost = Option<u32>;

#[derive(Debug, Resource)]
struct HexGrid {
    pub entities: HexagonalMap<(Cost, Entity)>,
    pub reachable_entities: HashSet<Entity>,
    pub layout: HexLayout,
}

/// 2D camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Input interaction
fn handle_input(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut tile_transforms: Query<(Entity, &mut Transform)>,
    mut current: Local<Hex>,
    mut grid: ResMut<HexGrid>,
) -> Result {
    let window = windows.single()?;
    let (camera, cam_transform) = cameras.single()?;
    if let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p).ok())
    {
        let hex_pos = grid.layout.world_pos_to_hex(pos);

        if hex_pos == *current {
            return Ok(());
        }
        *current = hex_pos;

        let field_of_movement =
            field_of_movement(hex_pos, BUDGET, |h| grid.entities.get(h).and_then(|c| c.0));

        let reachable_entities: HashSet<_> = field_of_movement
            .into_iter()
            .filter_map(|h| grid.entities.get(h).map(|&(_, ent)| ent))
            .collect();
        for (entity, mut transform) in tile_transforms.iter_mut() {
            if reachable_entities.contains(&entity) {
                *transform = transform.with_scale(Vec3::splat(0.9));
            } else {
                *transform = transform.with_scale(Vec3::splat(1.));
            }
        }

        grid.reachable_entities = reachable_entities;
    }
    Ok(())
}

fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let layout = HexLayout {
        scale: HEX_SIZE,
        ..default()
    };
    let mesh = meshes.add(hexagonal_plane(&layout));
    let plains_mat = materials.add(Color::Srgba(WHITE));
    let forest_mat = materials.add(Color::Srgba(LIME));
    let desert_mat = materials.add(Color::Srgba(YELLOW));
    let wall_mat = materials.add(Color::Srgba(GRAY));

    let mut rng = rand::rng();

    let entities = HexagonalMap::new(Hex::ZERO, MAP_RADIUS, |coord| {
        let cost = rng.random_range(0..=3);
        let pos = layout.hex_to_world_pos(coord);
        let material = match cost {
            0 => plains_mat.clone(),
            1 => forest_mat.clone(),
            2 => desert_mat.clone(),
            3 => wall_mat.clone(),
            _ => unreachable!(),
        };
        let cost = if (0..3).contains(&cost) {
            Some(cost)
        } else {
            None
        };
        let entity = commands
            .spawn((
                Mesh2d(mesh.clone()),
                MeshMaterial2d(material),
                Transform::from_xyz(pos.x, pos.y, 0.0),
            ))
            .id();
        (cost, entity)
    });
    commands.insert_resource(HexGrid {
        entities,
        reachable_entities: Default::default(),
        layout,
    })
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
