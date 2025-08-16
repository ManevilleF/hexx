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
        .insert_resource(
            RectMetadata::default()
                .with_hex_layout(
                    HexLayout::pointy()
                        .with_hex_size(30.0)
                        .with_origin(Vec2::ZERO),
                )
                .with_half_size(IVec2 { x: 8, y: 4 })
                .with_wrap_strategies([WrapStrategy::Cycle, WrapStrategy::Clamp]),
        )
        .init_resource::<CursorHex>()
        .add_event::<RespawnMap>()
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_input, respawn_map, cursor_draw).chain())
        .run();
}

#[derive(Resource)]
pub struct DefaultMaterial(pub [MeshMaterial2d<ColorMaterial>; 2]);

#[derive(Resource, Default)]
pub struct CursorHex(pub Option<Hex>);

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
    let default_mat = MeshMaterial2d(materials.add(ColorMaterial::from_color(GRAY)));
    let cursor_mat = MeshMaterial2d(materials.add(ColorMaterial::from_color(GREEN)));
    commands.insert_resource(DefaultMaterial([default_mat, cursor_mat]));
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

    wtr.write(RespawnMap);
}

fn respawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    meta: Res<RectMetadata>,
    default_mat: Res<DefaultMaterial>,
    query: Query<Entity, With<Hex>>,
    mut rdr: EventReader<RespawnMap>,
    mut text: Query<&mut Text, With<TextInstruction>>,
) {
    if rdr.read().count() == 0 {
        return;
    }

    let hex_size = 0.5 * meta.rect_size().max_element();
    let wrap_strategies = meta.wrap_strategies();

    let instructions = vec![
        format!("hex size : {}", hex_size),
        format!("half size: {:?}", meta.half_size()),
        format!("Press <1> to change orientation: {:?}", meta.orientation),
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
    ]
    .join("\n");

    *text.single_mut().unwrap() = Text::new(instructions);

    for i in query.iter() {
        commands.entity(i).despawn();
    }

    let default_mesh = Mesh2d(meshes.add(RegularPolygon::new(0.9 * hex_size, 6)));

    let angle = if meta.orientation == HexOrientation::Pointy {
        0.0
    } else {
        30_f32.to_radians()
    };

    let map: RectMap<Entity> = meta.clone().build(|hex| {
        let mut pos = meta.hex_to_world_pos(hex).xyy();
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
    mut meta: ResMut<RectMetadata>,
    mut wtr: EventWriter<RespawnMap>,
) {
    let mut changed = false;
    let mut orientation = meta.orientation;
    let mut hex_size = 0.5 * meta.rect_size().max_element();
    let mut half_size: IVec2 = meta.half_size();
    let mut wrap_strategies: [WrapStrategy; 2] = meta.wrap_strategies();

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

    if changed {
        let layout = if orientation == HexOrientation::Flat {
            HexLayout::flat()
        } else {
            HexLayout::pointy()
        }
        .with_hex_size(hex_size);

        *meta = RectMetadata::default()
            .with_hex_layout(layout)
            .with_half_size(half_size)
            .with_wrap_strategies(wrap_strategies);

        wtr.write(RespawnMap);
    }
}

fn cursor_draw(
    mut gizmos: Gizmos,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    map: Res<RectMap<Entity>>,
    default_mat: Res<DefaultMaterial>,
    mut query: Query<&mut MeshMaterial2d<ColorMaterial>>,
    mut last: Local<Option<Hex>>,
) -> Result {
    if let Some(hex) = *last
        && let Ok(mut mat) = query.get_mut(map[hex])
    {
        *mat = default_mat.0[1].clone();
    };

    let window = windows.single()?;
    let (camera, cam_transform) = cameras.single()?;
    if let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p).ok())
    {
        // draw small red circle in cursor position
        gizmos.circle_2d(Isometry2d::from_translation(pos), 5.0, RED);

        let hex = map.world_pos_to_hex(pos);
        let pos = map.hex_to_world_pos(hex);

        // draw large red circle in snapped to current hex's center.
        gizmos.circle_2d(
            Isometry2d::from_translation(pos),
            0.9 * 0.5 * map.rect_size().max_element(),
            RED,
        );

        let wrapped_hex = map.wrap_hex(hex);
        *last = Some(wrapped_hex);
        if let Ok(mut mat) = query.get_mut(map[wrapped_hex]) {
            *mat = default_mat.0[0].clone();
        }
    }
    Ok(())
}
