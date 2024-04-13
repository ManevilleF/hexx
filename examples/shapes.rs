use std::ops::DerefMut;

use bevy::{
    ecs::system::RunSystemOnce,
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::bevy_inspector;
use hexx::{shapes, *};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_000.0, 1_000.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin)
        .add_systems(Startup, (setup, generate).chain())
        .add_systems(Update, show_ui)
        .run();
}

#[derive(Reflect)]
#[reflect(Default)]
struct Hexagon {
    center: Hex,
    radius: u32,
}

impl Default for Hexagon {
    fn default() -> Self {
        Self {
            center: Hex::ZERO,
            radius: 20,
        }
    }
}

#[derive(Reflect)]
#[reflect(Default)]
struct Rombus {
    origin: Hex,
    rows: u32,
    columns: u32,
}

impl Default for Rombus {
    fn default() -> Self {
        Self {
            origin: Hex::ZERO,
            rows: 10,
            columns: 10,
        }
    }
}

#[derive(Reflect)]
#[reflect(Default)]
struct Rectangle {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            left: -10,
            right: 10,
            top: -10,
            bottom: 10,
        }
    }
}

#[derive(Reflect)]
#[reflect(Default)]
struct Triangle {
    size: u32,
}

impl Default for Triangle {
    fn default() -> Self {
        Self { size: 20 }
    }
}

#[derive(Reflect)]
#[reflect(Default)]
struct Parallelogram {
    min: Hex,
    max: Hex,
}

impl Default for Parallelogram {
    fn default() -> Self {
        Self {
            min: Hex::splat(-10),
            max: Hex::splat(10),
        }
    }
}

#[derive(Resource)]
struct HexMap {
    layout: HexLayout,
    entity: Entity,
    mat: Handle<ColorMaterial>,
}

#[derive(Resource)]
enum Shape {
    Hexagon(Hexagon),
    Rombus(Rombus),
    Triangle(Triangle),
    FlatRectangle(Rectangle),
    PointyRectangle(Rectangle),
    Parallelogram(Parallelogram),
}

impl Shape {
    fn all_values() -> [Self; 6] {
        [
            Self::Hexagon(Default::default()),
            Self::Rombus(Default::default()),
            Self::Triangle(Default::default()),
            Self::FlatRectangle(Default::default()),
            Self::PointyRectangle(Default::default()),
            Self::Parallelogram(Default::default()),
        ]
    }
    fn label(&self) -> &'static str {
        match self {
            Shape::Hexagon(_) => "Hexagon",
            Shape::Rombus(_) => "Rombus",
            Shape::Triangle(_) => "Triangle",
            Shape::FlatRectangle(_) => "FlatRectangle",
            Shape::PointyRectangle(_) => "PointyRectangle",
            Shape::Parallelogram(_) => "Parallelogram",
        }
    }
    fn coords(&self) -> Vec<Hex> {
        match self {
            Self::Hexagon(h) => shapes::hexagon(h.center, h.radius).collect(),
            Self::Rombus(r) => shapes::rombus(r.origin, r.rows, r.columns).collect(),
            Self::Triangle(t) => shapes::triangle(t.size).collect(),
            Self::FlatRectangle(r) => {
                shapes::flat_rectangle([r.left, r.right, r.top, r.bottom]).collect()
            }
            Self::PointyRectangle(r) => {
                shapes::pointy_rectangle([r.left, r.right, r.top, r.bottom]).collect()
            }
            Self::Parallelogram(p) => shapes::parallelogram(p.min, p.max).collect(),
        }
    }
}

pub fn setup(mut commands: Commands, mut mats: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    let layout = HexLayout {
        hex_size: Vec2::splat(10.0),
        ..default()
    };
    let mat = mats.add(Color::WHITE);
    let entity = commands.spawn(SpatialBundle::default()).id();
    commands.insert_resource(HexMap {
        layout,
        mat,
        entity,
    });
    commands.insert_resource(Shape::Hexagon(Default::default()));
}

fn show_ui(world: &mut World) {
    let mut regenerate = false;

    let Ok(egui_context) = world.query::<&mut EguiContext>().get_single(world) else {
        return;
    };
    let mut egui_context = egui_context.clone();
    egui::Window::new("Options").show(egui_context.get_mut(), |ui| {
        world.resource_scope(|world, mut map: Mut<HexMap>| {
            bevy_inspector::ui_for_value(&mut map.layout.orientation, ui, world);
        });
        world.resource_scope(|world, mut shape: Mut<Shape>| {
            egui::ComboBox::from_id_source("Shape")
                .selected_text(shape.label())
                .show_ui(ui, |ui| {
                    for option in Shape::all_values() {
                        if ui.selectable_label(false, option.label()).clicked() {
                            *shape = option;
                        };
                    }
                });
            match shape.deref_mut() {
                Shape::Hexagon(v) => bevy_inspector::ui_for_value(v, ui, world),
                Shape::Rombus(v) => bevy_inspector::ui_for_value(v, ui, world),
                Shape::Triangle(v) => bevy_inspector::ui_for_value(v, ui, world),
                Shape::FlatRectangle(v) => bevy_inspector::ui_for_value(v, ui, world),
                Shape::PointyRectangle(v) => bevy_inspector::ui_for_value(v, ui, world),
                Shape::Parallelogram(v) => bevy_inspector::ui_for_value(v, ui, world),
            };

            regenerate = ui.button("Generate").clicked();
        });
    });
    if regenerate {
        world.run_system_once(generate);
    }
}

fn generate(
    mut commands: Commands,
    map: Res<HexMap>,
    shape: Res<Shape>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.entity(map.entity).despawn_descendants();
    let mesh = meshes.add(hexagonal_plane(&map.layout));
    for coord in shape.coords() {
        let pos = map.layout.hex_to_world_pos(coord);
        commands
            .spawn(ColorMesh2dBundle {
                mesh: mesh.clone().into(),
                material: map.mat.clone_weak(),
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                ..default()
            })
            .set_parent(map.entity);
    }
}

/// Compute a bevy mesh from the layout
fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(Vec3::Z)
        .with_scale(Vec3::splat(0.9))
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
