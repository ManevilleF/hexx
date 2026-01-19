use bevy::{color::palettes::css::*, prelude::*, window::PrimaryWindow};

use hexx::{
    storage::{RectMap, RectMetadata, WrapStrategy},
    *,
};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_000, 1_000).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<CursorPos>()
        .add_message::<RespawnMap>()
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
    orientation: HexOrientation,
    hex_size: f32,
    wrap_strategies: [WrapStrategy; 2],
    offset_mode: OffsetHexMode,
    half_size: UVec2,
    start: IVec2,
    end: IVec2,
    dim: UVec2,
    mode: RectCreateMode,
}

pub enum RectCreateMode {
    HalfSize,
    StartEnd,
    StartDim,
}

#[derive(Resource)]
pub struct DefaultMaterial(pub [MeshMaterial2d<ColorMaterial>; 2]);

#[derive(Resource, Default)]
pub struct CursorPos(pub Option<Vec2>);

#[derive(Component)]
pub struct TextInstruction;

#[derive(Message)]
pub struct RespawnMap;

/// setup
fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut wtr: MessageWriter<RespawnMap>,
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

    // commands.insert_resource(RectMapConfig {
    //     meta: RectMetadata::from_half_size(UVec2::new(8, 4))
    //         .with_offset_mode(OffsetHexMode::Odd)
    //         .with_orientation(HexOrientation::Pointy)
    //         .with_wrap_strategies([WrapStrategy::Cycle, WrapStrategy::Clamp]),
    //     layout: HexLayout::pointy().with_hex_size(30.0),
    // });
    commands.insert_resource(RectMapConfig {
        orientation: HexOrientation::Pointy,
        hex_size: 30.0,
        wrap_strategies: [WrapStrategy::Cycle, WrapStrategy::Clamp],
        offset_mode: OffsetHexMode::Odd,
        half_size: UVec2 { x: 8, y: 4 },
        start: IVec2 { x: -8, y: -4 },
        end: IVec2 { x: 8, y: 4 },
        dim: UVec2 { x: 16, y: 8 },
        mode: RectCreateMode::HalfSize,
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
    mut rdr: MessageReader<RespawnMap>,
) {
    if rdr.read().count() == 0 {
        return;
    }

    for i in query.iter() {
        commands.entity(i).despawn();
    }

    let mut instructions = vec![
        format!("Press <1> to change orientation: {:?}", config.orientation),
        format!("Press <2,3> to change hex size: {:?}", config.hex_size),
        format!(
            "Press <4,5> to change longitude wrapping strategy: {:?}, {:?}",
            config.wrap_strategies[0], config.wrap_strategies[1]
        ),
        format!(
            "Press <6> to change the offset mode: {:?}",
            config.offset_mode
        ),
        format!(""),
        format!("Press <Tab> to change rect map creation mode"),
    ];

    match config.mode {
        RectCreateMode::HalfSize => instructions.append(&mut vec![
            format!("half size: {:?}", config.half_size),
            format!("Press <w,a,s,d>> to change half size"),
        ]),

        RectCreateMode::StartEnd => instructions.append(&mut vec![
            format!("start, end: {:?}, {:?}", config.start, config.end),
            format!("Press <w,a,s,d> to change start"),
            format!("Press <arrows> to change end"),
        ]),
        RectCreateMode::StartDim => instructions.append(&mut vec![
            format!("start, dim: {:?}, {:?}", config.start, config.dim),
            format!("Press <w,a,s,d> to change start"),
            format!("Press <arrows> to change dimension"),
        ]),
    }

    let instructions = instructions.join("\n");

    *text.single_mut().unwrap() = Text::new(instructions);

    let default_mesh = Mesh2d(meshes.add(RegularPolygon::new(0.9 * config.hex_size, 6)));

    let angle = if config.orientation == HexOrientation::Pointy {
        0.0
    } else {
        30_f32.to_radians()
    };

    let meta = match config.mode {
        RectCreateMode::HalfSize => RectMetadata::from_half_size(config.half_size),
        RectCreateMode::StartEnd => RectMetadata::from_start_end(config.start, config.end),
        RectCreateMode::StartDim => RectMetadata::from_start_dim(config.start, config.dim),
    }
    .with_orientation(config.orientation)
    .with_wrap_strategies(config.wrap_strategies)
    .with_offset_mode(config.offset_mode);

    let layout = HexLayout::new(config.orientation).with_hex_size(config.hex_size);

    let map: RectMap<Entity> = meta.clone().build(|hex| {
        let mut pos = layout.hex_to_world_pos(hex).xyy();
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
    mut wtr: MessageWriter<RespawnMap>,
) {
    if key.just_pressed(KeyCode::Tab) {
        config.mode = match config.mode {
            RectCreateMode::HalfSize => RectCreateMode::StartEnd,
            RectCreateMode::StartEnd => RectCreateMode::StartDim,
            RectCreateMode::StartDim => RectCreateMode::HalfSize,
        };
    }

    if key.just_pressed(KeyCode::Digit1) {
        if config.orientation == HexOrientation::Flat {
            config.orientation = HexOrientation::Pointy
        } else {
            config.orientation = HexOrientation::Flat
        }
    }
    if key.just_pressed(KeyCode::Digit2) {
        config.hex_size = (config.hex_size + 1.0).clamp(15.0, 45.0);
    }
    if key.just_pressed(KeyCode::Digit3) {
        config.hex_size = (config.hex_size - 1.0).clamp(15.0, 45.0);
    }
    if key.just_pressed(KeyCode::Digit4) {
        if config.wrap_strategies[0] == WrapStrategy::Clamp {
            config.wrap_strategies[0] = WrapStrategy::Cycle;
        } else {
            config.wrap_strategies[0] = WrapStrategy::Clamp;
        }
    }
    if key.just_pressed(KeyCode::Digit5) {
        if config.wrap_strategies[1] == WrapStrategy::Clamp {
            config.wrap_strategies[1] = WrapStrategy::Cycle;
        } else {
            config.wrap_strategies[1] = WrapStrategy::Clamp;
        }
    }
    if key.just_pressed(KeyCode::Digit6) {
        if config.offset_mode == OffsetHexMode::Odd {
            config.offset_mode = OffsetHexMode::Even;
        } else {
            config.offset_mode = OffsetHexMode::Odd
        }
    }

    match config.mode {
        RectCreateMode::HalfSize => {
            if key.just_pressed(KeyCode::KeyD) {
                config.half_size[0] = (config.half_size[0] + 1).clamp(2, 16);
            }
            if key.just_pressed(KeyCode::KeyA) {
                config.half_size[0] = (config.half_size[0] - 1).clamp(2, 16);
            }
            if key.just_pressed(KeyCode::KeyW) {
                config.half_size[1] = (config.half_size[1] + 1).clamp(1, 8);
            }
            if key.just_pressed(KeyCode::KeyS) {
                config.half_size[1] = (config.half_size[1] - 1).clamp(1, 8);
            }
        }
        RectCreateMode::StartEnd => {
            if key.just_pressed(KeyCode::KeyD) {
                config.start[0] = (config.start[0] + 1).clamp(-8, 0);
            }
            if key.just_pressed(KeyCode::KeyA) {
                config.start[0] = (config.start[0] - 1).clamp(-8, 0);
            }
            if key.just_pressed(KeyCode::KeyW) {
                config.start[1] = (config.start[1] + 1).clamp(-8, 0);
            }
            if key.just_pressed(KeyCode::KeyS) {
                config.start[1] = (config.start[1] - 1).clamp(-8, 0);
            }
            //
            if key.just_pressed(KeyCode::ArrowRight) {
                config.end[0] = (config.end[0] + 1).clamp(0, 8);
            }
            if key.just_pressed(KeyCode::ArrowLeft) {
                config.end[0] = (config.end[0] - 1).clamp(0, 8);
            }
            if key.just_pressed(KeyCode::ArrowUp) {
                config.end[1] = (config.end[1] + 1).clamp(0, 4);
            }
            if key.just_pressed(KeyCode::ArrowDown) {
                config.end[1] = (config.end[1] - 1).clamp(0, 4);
            }
        }
        RectCreateMode::StartDim => {
            if key.just_pressed(KeyCode::KeyD) {
                config.start[0] = (config.start[0] + 1).clamp(-8, 0);
            }
            if key.just_pressed(KeyCode::KeyA) {
                config.start[0] = (config.start[0] - 1).clamp(-8, 0);
            }
            if key.just_pressed(KeyCode::KeyW) {
                config.start[1] = (config.start[1] + 1).clamp(-8, 0);
            }
            if key.just_pressed(KeyCode::KeyS) {
                config.start[1] = (config.start[1] - 1).clamp(-8, 0);
            }
            //
            if key.just_pressed(KeyCode::ArrowRight) {
                config.dim[0] = (config.dim[0] + 1).clamp(1, 24);
            }
            if key.just_pressed(KeyCode::ArrowLeft) {
                config.dim[0] = (config.dim[0] - 1).clamp(1, 24);
            }
            if key.just_pressed(KeyCode::ArrowUp) {
                config.dim[1] = (config.dim[1] + 1).clamp(1, 16);
            }
            if key.just_pressed(KeyCode::ArrowDown) {
                config.dim[1] = (config.dim[1] - 1).clamp(1, 16);
            }
        }
    }

    if key.get_just_pressed().count() != 0 {
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

    let layout = HexLayout::new(config.orientation).with_hex_size(config.hex_size);

    let hex = layout.world_pos_to_hex(pos);
    let pos = layout.hex_to_world_pos(hex);

    // draw large red circle in snapped to current hex's center.
    gizmos.circle_2d(
        Isometry2d::from_translation(pos),
        0.9 * config.hex_size,
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
