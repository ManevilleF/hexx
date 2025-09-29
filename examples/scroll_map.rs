use bevy::{
    asset::RenderAssetUsages, mesh::Indices, platform::collections::HashMap, prelude::*, render::render_resource::PrimitiveTopology, window::PrimaryWindow
};
use hexx::*;

/// World size of the hexagons (outer radius)
const HEX_SIZE: Vec2 = Vec2::splat(15.0);
const MAP_RADIUS: u32 = 10;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_000, 1_000).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_grid))
        .add_systems(Update, handle_input)
        .run();
}

#[derive(Debug, Resource)]
struct HexGrid {
    pub entities: HashMap<Hex, Entity>,
    pub layout: HexLayout,
    pub bounds: HexBounds,
}

/// 2D camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
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

    let bounds = HexBounds::new(Hex::ZERO, MAP_RADIUS);
    let entities = bounds
        .all_coords()
        .map(|hex| {
            let v = 1.0 - (hex.length() as f32 / MAP_RADIUS as f32);
            let color = Color::srgb(v, v, v);
            let material = materials.add(color);
            let pos = layout.hex_to_world_pos(hex);
            let entity = commands
                .spawn((
                    Mesh2d(mesh.clone()),
                    MeshMaterial2d(material),
                    Transform::from_xyz(pos.x, pos.y, 0.0),
                ))
                .id();
            (hex, entity)
        })
        .collect();
    commands.insert_resource(HexGrid {
        entities,
        layout,
        bounds,
    })
}

/// Input interaction
fn handle_input(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    grid: Res<HexGrid>,
) -> Result {
    let window = windows.single()?;
    let (camera, cam_transform) = cameras.single()?;
    if let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p).ok())
    {
        let hex_pos = grid.layout.world_pos_to_hex(pos);
        for h in hex_pos.range(grid.bounds.radius) {
            let wrapped = grid.bounds.wrap(h);
            let pos = grid.layout.hex_to_world_pos(h);
            commands
                .entity(grid.entities[&wrapped])
                .insert(Transform::from_xyz(pos.x, pos.y, 0.0));
        }
    }
    Ok(())
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
