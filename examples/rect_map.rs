use bevy::{color::palettes::css::*, prelude::*, window::PrimaryWindow};

use hexx::{
    storage::{RectMap, RectMetadata, WrapStrategy},
    *,
};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_000.0, 1_000.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<CursorPos>()
        .add_event::<RespawnMap>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_input,
                respawn_map,
                update_cursor_pos,
                update_material,
            )
                .chain(),
        )
        .run();
}

#[derive(Resource)]
pub struct RectMapConfig {
    pub meta: RectMetadata,
    pub layout: HexLayout,
}

#[derive(Resource)]
pub struct DefaultMaterial(pub [MeshMaterial2d<ColorMaterial>; 2]);

#[derive(Resource, Default)]
pub struct CursorPos(pub Option<Vec2>);

#[derive(Component)]
pub struct TextInstruction;

#[derive(Event)]
pub struct RespawnMap;

/// setup
fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut wtr: EventWriter<RespawnMap>,
) {
    commands.spawn(Camera2d);
    commands.insert_resource(DefaultMaterial([
        MeshMaterial2d(materials.add(ColorMaterial::from_color(GRAY))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(GREEN))),
    ]));
    commands.spawn((
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },
        TextInstruction,
    ));

    commands.insert_resource(RectMapConfig {
        meta: RectMetadata::new(IVec2::new(8, 4))
            .with_offset_mode(OffsetHexMode::Odd)
            .with_orientation(HexOrientation::Pointy)
            .with_wrap_strategies([WrapStrategy::Cycle, WrapStrategy::Clamp]),
        layout: HexLayout::pointy().with_hex_size(30.0),
    });

    wtr.write(RespawnMap);
}

fn respawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    config: Res<RectMapConfig>,
    default_mat: Res<DefaultMaterial>,
    query: Query<Entity, With<Hex>>,
    mut text: Query<&mut Text, With<TextInstruction>>,
    mut rdr: EventReader<RespawnMap>,
) {
    if rdr.read().count() == 0 {
        return;
    }

    for i in query.iter() {
        commands.entity(i).despawn();
    }

    let hex_size = 0.5 * config.layout.rect_size().max_element();
    let wrap_strategies = config.meta.wrap_strategies();

    let instructions = vec![
        format!("hex size : {}", hex_size),
        format!("half size: {:?}", config.meta.half_size()),
        format!(
            "Press <1> to change orientation: {:?}",
            config.meta.orientation()
        ),
        format!("Press <2> to increase hex size"),
        format!("Press <3> to decrease hex size"),
        format!("Press <4> to increase column count"),
        format!("Press <5> to decrease column count"),
        format!("Press <6> to increase row count"),
        format!("Press <7> to decrease column count"),
        format!(
            "Press <8> to change longitude wrapping strategy: {:?}",
            wrap_strategies[0]
        ),
        format!(
            "Press <9> to change latitude wrapping strategy: {:?}",
            wrap_strategies[1]
        ),
        format!(
            "Press <0> to change the offset mode: {:?}",
            config.meta.offset_mode()
        ),
    ]
    .join("\n");

    *text.single_mut().unwrap() = Text::new(instructions);

    let default_mesh = Mesh2d(meshes.add(RegularPolygon::new(0.9 * hex_size, 6)));

    let angle = if config.meta.orientation() == HexOrientation::Pointy {
        0.0
    } else {
        30_f32.to_radians()
    };

    let map: RectMap<Entity> = config.meta.clone().build(|hex| {
        let mut pos = config.layout.hex_to_world_pos(hex).xyy();
        pos[2] = 0.0;

        commands
            .spawn((
                hex,
                Transform {
                    translation: pos,
                    rotation: Quat::from_rotation_z(angle),
                    ..Default::default()
                },
                default_mat.0[0].clone(),
                default_mesh.clone(),
            ))
            .id()
    });

    // Insert RectMap<Entity> as an resource
    commands.insert_resource(map);
}

fn handle_input(
    key: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<RectMapConfig>,
    mut wtr: EventWriter<RespawnMap>,
) {
    let mut changed = false;
    let mut orientation = config.meta.orientation();
    let mut hex_size = 0.5 * config.layout.rect_size().max_element();
    let mut half_size: IVec2 = config.meta.half_size();
    let mut wrap_strategies: [WrapStrategy; 2] = config.meta.wrap_strategies();
    let mut offset_mode = config.meta.offset_mode();

    if key.just_pressed(KeyCode::Digit1) {
        if orientation == HexOrientation::Flat {
            orientation = HexOrientation::Pointy
        } else {
            orientation = HexOrientation::Flat
        }
        changed = true;
    }
    if key.just_pressed(KeyCode::Digit2) {
        hex_size = (hex_size + 1.0).clamp(15.0, 45.0);
        changed = true;
    }
    if key.just_pressed(KeyCode::Digit3) {
        hex_size = (hex_size - 1.0).clamp(15.0, 45.0);
        changed = true;
    }
    if key.just_pressed(KeyCode::Digit4) {
        half_size[0] = (half_size[0] + 1).clamp(4, 16);
        changed = true;
    }
    if key.just_pressed(KeyCode::Digit5) {
        half_size[0] = (half_size[0] - 1).clamp(4, 16);
        changed = true;
    }
    if key.just_pressed(KeyCode::Digit6) {
        half_size[1] = (half_size[1] + 1).clamp(2, 8);
        changed = true;
    }
    if key.just_pressed(KeyCode::Digit7) {
        half_size[1] = (half_size[1] - 1).clamp(2, 8);
        changed = true;
    }
    if key.just_pressed(KeyCode::Digit8) {
        if wrap_strategies[0] == WrapStrategy::Clamp {
            wrap_strategies[0] = WrapStrategy::Cycle;
        } else {
            wrap_strategies[0] = WrapStrategy::Clamp;
        }
        changed = true;
    }
    if key.just_pressed(KeyCode::Digit9) {
        if wrap_strategies[1] == WrapStrategy::Clamp {
            wrap_strategies[1] = WrapStrategy::Cycle;
        } else {
            wrap_strategies[1] = WrapStrategy::Clamp;
        }
        changed = true;
    }
    if key.just_pressed(KeyCode::Digit0) {
        if offset_mode == OffsetHexMode::Odd {
            offset_mode = OffsetHexMode::Even;
        } else {
            offset_mode = OffsetHexMode::Odd
        }
        changed = true;
    }

    if changed {
        config.meta = RectMetadata::new(half_size)
            .with_orientation(orientation)
            .with_wrap_strategies(wrap_strategies)
            .with_offset_mode(offset_mode);

        config.layout = HexLayout::new(orientation).with_hex_size(hex_size);

        wtr.write(RespawnMap);
    }
}

fn update_material(
    mut gizmos: Gizmos,
    map: Res<RectMap<Entity>>,
    config: Res<RectMapConfig>,
    default_mat: Res<DefaultMaterial>,
    cursor_pos: ResMut<CursorPos>,
    mut query: Query<&mut MeshMaterial2d<ColorMaterial>>,
    mut last: Local<Option<Hex>>,
) {
    let Some(pos) = cursor_pos.0 else {
        return;
    };
    if let Some(hex) = *last
        && map.contains_hex(hex)
        && let Ok(mut mat) = query.get_mut(map[hex])
    {
        *mat = default_mat.0[0].clone();
    };
    // draw small red circle in cursor position
    gizmos.circle_2d(Isometry2d::from_translation(pos), 5.0, RED);

    let hex = config.layout.world_pos_to_hex(pos);
    let pos = config.layout.hex_to_world_pos(hex);

    // draw large red circle in snapped to current hex's center.
    gizmos.circle_2d(
        Isometry2d::from_translation(pos),
        0.9 * 0.5 * config.layout.rect_size().max_element(),
        RED,
    );

    let wrapped_hex = map.wrap_hex(hex);
    *last = Some(wrapped_hex);
    if let Ok(mut mat) = query.get_mut(map[wrapped_hex]) {
        *mat = default_mat.0[1].clone();
    }
}

fn update_cursor_pos(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut cursor_pos: ResMut<CursorPos>,
) -> Result {
    let window = windows.single()?;
    let (camera, cam_transform) = cameras.single()?;
    if let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p).ok())
    {
        cursor_pos.0 = Some(pos);
    } else {
        cursor_pos.0 = None;
    }
    Ok(())
}
