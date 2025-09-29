use bevy::{
    color::palettes::css::{AQUA, ORANGE, VIOLET},
    platform::collections::HashSet,
    prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::{InspectorOptions, quick::ResourceInspectorPlugin};
use hexx::*;

pub fn main() {
    App::new()
        .register_type::<ChunkSettings>()
        .register_type::<MapSettings>()
        .init_resource::<MapSettings>()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(ResourceInspectorPlugin::<MapSettings>::default())
        .add_systems(Startup, setup_camera)
        .add_systems(Update, setup_grid)
        .run();
}

#[derive(Debug, Resource, Reflect, InspectorOptions)]
struct ChunkSettings {
    #[inspector(min = 1, max = 10)]
    pub size: u32,
    pub color: Color,
}

/// Egui settings
#[derive(Debug, Resource, Reflect, InspectorOptions)]
struct MapSettings {
    #[inspector(min = 1.0, max = 30.0)]
    pub hex_size: f32,
    #[inspector(min = 10, max = 100)]
    pub map_size: u32,
    pub chunks: Vec<ChunkSettings>,
}

/// 2D camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Hex grid setup
fn setup_grid(settings: Res<MapSettings>, mut gizmos: Gizmos) {
    let layout = HexLayout::flat().with_hex_size(settings.hex_size);
    let size = settings.map_size as i32;
    let mut drawn = HashSet::new();
    for hex in shapes::flat_rectangle([-size - size / 2, size + size / 2, -size, size]) {
        let mut corners = layout.hex_corners(hex).to_vec();
        corners.push(corners[0]);
        gizmos.linestrip_2d(corners, Color::WHITE);
        for chunk in &settings.chunks {
            let chunk_coord = hex.to_lower_res(chunk.size);
            let chunk_center = chunk_coord.to_higher_res(chunk.size);
            if drawn.contains(&(chunk_center, chunk.size)) {
                continue;
            }
            let mut positions = EdgeDirection::ALL_DIRECTIONS
                .map(|d| layout.hex_to_world_pos(chunk_center + (d * chunk.size as i32)))
                .to_vec();
            positions.push(positions[0]);
            gizmos.linestrip_2d(positions, chunk.color);
            drawn.insert((chunk_center, chunk.size));
        }
    }
}

impl Default for MapSettings {
    fn default() -> Self {
        Self {
            hex_size: 8.0,
            map_size: 15,
            chunks: vec![
                ChunkSettings {
                    size: 2,
                    color: AQUA.into(),
                },
                ChunkSettings {
                    size: 5,
                    color: ORANGE.into(),
                },
                ChunkSettings {
                    size: 10,
                    color: VIOLET.into(),
                },
            ],
        }
    }
}
