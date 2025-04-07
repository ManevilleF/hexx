use bevy::{platform_support::collections::hash_map::HashMap, prelude::*, window::PrimaryWindow};
use glam::uvec2;
use hexx::{shapes, *};

// 10% of the real individual texture sizes
const SPRITE_SIZE: Vec2 = Vec2::new(24.0, 28.0);

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

/// 2D camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Debug, Resource)]
struct HexGrid {
    pub entities: HashMap<Hex, Entity>,
    pub layout: HexLayout,
}

fn setup_grid(
    mut commands: Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("kenney/hexagonTerrain_sheet.png");
    let atlas_layout =
        TextureAtlasLayout::from_grid(uvec2(120, 140), 7, 6, Some(uvec2(2, 2)), None);
    let atlas_layout = atlas_layouts.add(atlas_layout);
    let layout = HexLayout::new(HexOrientation::Pointy).with_rect_size(SPRITE_SIZE);
    let entities = shapes::pointy_rectangle([-14, 14, -16, 16])
        .enumerate()
        .map(|(i, coord)| {
            let pos = layout.hex_to_world_pos(coord);
            let index = i % (7 * 6);
            let entity = commands
                .spawn((
                    Sprite {
                        custom_size: Some(SPRITE_SIZE),
                        image: texture.clone(),
                        texture_atlas: Some(TextureAtlas {
                            index,
                            layout: atlas_layout.clone(),
                        }),
                        ..default()
                    },
                    Transform::from_xyz(pos.x, pos.y, 0.0),
                ))
                .id();
            (coord, entity)
        })
        .collect();
    commands.insert_resource(HexGrid { entities, layout });
}

/// Input interaction
fn handle_input(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    grid: Res<HexGrid>,
    mut tiles: Query<&mut Sprite>,
) -> Result {
    let window = windows.single()?;
    let (camera, cam_transform) = cameras.single()?;
    if let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p).ok())
    {
        let hex_pos = grid.layout.world_pos_to_hex(pos);
        let Some(entity) = grid.entities.get(&hex_pos).copied() else {
            return Ok(());
        };
        if !buttons.just_pressed(MouseButton::Left) {
            return Ok(());
        }
        let Ok(mut sprite) = tiles.get_mut(entity) else {
            return Ok(());
        };
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = (atlas.index + 1) % (7 * 6);
        }
    }
    Ok(())
}
