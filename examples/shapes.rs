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

/// World size of the hexagons (outer radius)
const HEX_SIZE: Vec2 = Vec2::splat(13.0);

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

#[derive(Resource)]
struct HexMap {
    layout: HexLayout,
    entity: Entity,
    mat: Handle<ColorMaterial>,
}

#[derive(Resource)]
enum Shape {
    Hexagon(shapes::Hexagon),
    Rombus(shapes::Rombus),
    Triangle(shapes::Triangle),
    FlatRectangle(shapes::FlatRectangle),
    PointyRectangle(shapes::PointyRectangle),
    Parallelogram(shapes::Parallelogram),
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
            Self::Hexagon(v) => v.coords().collect(),
            Self::Rombus(v) => v.coords().collect(),
            Self::Triangle(v) => v.coords().collect(),
            Self::FlatRectangle(v) => v.coords().collect(),
            Self::PointyRectangle(v) => v.coords().collect(),
            Self::Parallelogram(v) => v.coords().collect(),
        }
    }
}

pub fn setup(mut commands: Commands, mut mats: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2d);
    let layout = HexLayout {
        hex_size: HEX_SIZE,
        ..default()
    };
    let mat = mats.add(Color::WHITE);
    let entity = commands
        .spawn((Transform::default(), Visibility::default()))
        .id();
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
            ui.horizontal(|ui| {
                ui.label("Orientation");
                bevy_inspector::ui_for_value(&mut map.layout.orientation, ui, world);
            });
        });

        world.resource_scope(|world, mut shape: Mut<Shape>| {
            ui.horizontal(|ui| {
                ui.label("Shape");
                egui::ComboBox::from_id_salt("Shape")
                    .selected_text(shape.label())
                    .show_ui(ui, |ui| {
                        for option in Shape::all_values() {
                            if ui.selectable_label(false, option.label()).clicked() {
                                *shape = option;
                            };
                        }
                    });
            });
            match shape.deref_mut() {
                Shape::Hexagon(v) => bevy_inspector::ui_for_value(v, ui, world),
                Shape::Rombus(v) => bevy_inspector::ui_for_value(v, ui, world),
                Shape::Triangle(v) => bevy_inspector::ui_for_value(v, ui, world),
                Shape::FlatRectangle(v) => bevy_inspector::ui_for_value(v, ui, world),
                Shape::PointyRectangle(v) => bevy_inspector::ui_for_value(v, ui, world),
                Shape::Parallelogram(v) => bevy_inspector::ui_for_value(v, ui, world),
            };

            ui.add_space(10.0);
            ui.vertical_centered_justified(|ui| {
                regenerate = ui.button("Generate").clicked();
            });
        });
    });
    if regenerate {
        world.run_system_once(generate).unwrap();
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
            .spawn((
                Mesh2d(mesh.clone()),
                MeshMaterial2d(map.mat.clone_weak()),
                Transform::from_xyz(pos.x, pos.y, 0.0),
            ))
            .with_children(|b| {
                b.spawn((
                    Text2d(format!("{},{}", coord.x, coord.y)),
                    TextColor(Color::BLACK),
                    TextFont {
                        font_size: 7.0,
                        ..default()
                    },
                    Transform::from_xyz(0.0, 0.0, 10.0),
                ));
            })
            .set_parent(map.entity);
    }
}

/// Compute a bevy mesh from the layout
fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(Vec3::Z)
        .with_scale(Vec3::splat(0.98))
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
