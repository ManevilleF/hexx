use std::ops::Range;

use bevy::{
    color::palettes::css::{BLUE, RED, WHITE},
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};
use bevy_inspector_egui::{quick::ResourceInspectorPlugin, InspectorOptions};
use hexx::*;
use rand::{thread_rng, Rng};

/// Chunk colors
const COLORS: [Color; 3] = [Color::Srgba(BLUE), Color::Srgba(WHITE), Color::Srgba(RED)];

pub fn main() {
    App::new()
        .register_type::<MapSettings>()
        .register_type::<Range<f32>>()
        .init_resource::<MapSettings>()
        .insert_resource(AmbientLight {
            brightness: 500.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(ResourceInspectorPlugin::<MapSettings>::default())
        .add_systems(Startup, setup_camera)
        .add_systems(Update, setup_grid)
        .run();
}

/// Egui settings
#[derive(Debug, Resource, Reflect, InspectorOptions)]
struct MapSettings {
    pub hex_size: Vec2,
    pub column_heights: [f32; 2],
    #[inspector(min = 0, max = 50)]
    pub map_radius: u32,
    #[inspector(min = 1, max = 50)]
    pub chunk_radius: u32,
}

#[derive(Debug, Resource)]
struct Map(pub Entity);

/// 3D camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 60.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// Hex grid setup
fn setup_grid(
    settings: Res<MapSettings>,
    mut map: Local<Map>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !settings.is_changed() {
        return;
    }
    println!("Generating map");
    if map.0 != Entity::PLACEHOLDER {
        commands.entity(map.0).despawn_recursive();
    }
    let layout = HexLayout {
        hex_size: settings.hex_size,
        ..default()
    };
    // Materials shouldn't be added to assets every time, this is just to keep the
    // example simple
    let materials = COLORS.map(|c| materials.add(c));

    let map_entity = commands
        .spawn((
            Name::new("Chunks"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();
    let mut rng = thread_rng();
    // For each chunk
    for chunk in Hex::ZERO.range(settings.map_radius) {
        // We retrieve its center chil
        let center = chunk.to_higher_res(settings.chunk_radius);
        // Retrieve its world pos
        let pos = layout.hex_to_world_pos(center);
        // Compute the color index for the chunk
        let color_index = (chunk.x - chunk.y).rem_euclid(3) as usize;
        // Retrieve the local children coordinates (can be cached)
        let children = Hex::ZERO.range(settings.chunk_radius);
        // We compute the merged mesh with all children columns
        let mesh = children.fold(MeshInfo::default(), |mut mesh, c| {
            let [min, max] = settings.column_heights;
            let height = if min < max {
                rng.gen_range(min..=max)
            } else {
                min
            };
            let info = ColumnMeshBuilder::new(&layout, height)
                .at(c)
                .without_bottom_face()
                .center_aligned()
                .build();
            mesh.merge_with(info);
            mesh
        });
        let mesh = meshes.add(hex_mesh(mesh));
        commands
            .spawn((
                Name::new(format!("Chunk {} {}", chunk.x, chunk.y)),
                Mesh3d(mesh),
                MeshMaterial3d(materials[color_index].clone()),
                Transform::from_xyz(pos.x, 0.0, pos.y),
            ))
            .set_parent(map_entity);
    }
    map.0 = map_entity;
}

/// Compute a bevy mesh from a hexx mesh
fn hex_mesh(mesh_info: MeshInfo) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}

impl Default for MapSettings {
    fn default() -> Self {
        Self {
            hex_size: Vec2::ONE,
            map_radius: 10,
            chunk_radius: 5,
            column_heights: [5.0, 10.0],
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}
