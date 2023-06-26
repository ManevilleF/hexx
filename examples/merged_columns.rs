use std::ops::Range;

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use bevy_inspector_egui::InspectorOptions;
use hexx::*;
use rand::{thread_rng, Rng};

/// Chunk colors
const COLORS: [Color; 3] = [Color::WHITE, Color::BLUE, Color::RED];

pub fn main() {
    App::new()
        .register_type::<MapSettings>()
        .register_type::<Range<f32>>()
        .init_resource::<MapSettings>()
        .insert_resource(AmbientLight {
            brightness: 1.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_system(setup_grid)
        .add_plugin(ResourceInspectorPlugin::<MapSettings>::default())
        .add_plugin(WorldInspectorPlugin::default())
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

/// 3D Orthogrpahic camera setup
fn setup_camera(mut commands: Commands) {
    let transform = Transform::from_xyz(0.0, 60.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn(Camera3dBundle {
        transform,
        ..default()
    });
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
    // Materials shouldn't be added to assets every time, this is just to keep the example simple
    let materials = COLORS.map(|c| materials.add(c.into()));

    let map_entity = commands
        .spawn((SpatialBundle::default(), Name::new("Chunks")))
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
            let height = rng.gen_range(min..=max);
            let info = ColumnMeshBuilder::new(&layout, height).at(c).build();
            mesh.merge_with(info);
            mesh
        });
        let mesh = meshes.add(hex_mesh(mesh));
        commands
            .spawn((
                Name::new(format!("Chunk {} {}", chunk.x, chunk.y)),
                PbrBundle {
                    mesh,
                    material: materials[color_index].clone(),
                    transform: Transform::from_xyz(pos.x, 0.0, pos.y),
                    ..default()
                },
            ))
            .set_parent(map_entity);
    }
    map.0 = map_entity;
}

/// Compute a bevy mesh from a hexx mesh
fn hex_mesh(mesh_info: MeshInfo) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs);
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
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
