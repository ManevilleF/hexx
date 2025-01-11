use std::collections::HashMap;

use bevy::{
    ecs::system::RunSystemOnce,
    input::mouse::MouseMotion,
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};
use bevy_egui::{
    egui::{self, Ui},
    EguiContext, EguiPlugin,
};
use bevy_inspector_egui::bevy_inspector;
use hexx::*;
use rand::{thread_rng, Rng};
use storage::HexagonalMap;

#[derive(Debug, Resource)]
struct HexInfo {
    pub layout: HexLayout,
    pub mesh_entity: Entity,
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Handle<StandardMaterial>,
}

#[derive(Debug, Resource)]
struct BuilderParams {
    pub max_height: f32,
    pub range: u32,
    pub scale: Vec3,
    pub fill_holes: bool,
    pub sides_options: Option<FaceOptions>,
    pub caps_options: Option<FaceOptions>,
}

pub fn main() {
    App::new()
        .init_resource::<BuilderParams>()
        .insert_resource(AmbientLight {
            brightness: 500.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (show_ui, animate, gizmos))
        .run();
}

fn face_opts(ui: &mut Ui, options: &mut Option<FaceOptions>, world: &mut World) {
    if options.is_none() {
        if ui.button("Enable").clicked() {
            *options = Some(FaceOptions::new());
        }
    } else if ui.button("Disable").clicked() {
        *options = None;
    }
    let Some(opts) = options else {
        return;
    };

    ui.scope(|ui| {
        ui.label("UV");
        bevy_inspector::ui_for_value(&mut opts.uv, ui, world);
        ui.horizontal(|ui| {
            ui.label("Insetting");
            if opts.insetting.is_none() {
                if ui.button("Enable").clicked() {
                    opts.insetting = Some(InsetOptions {
                        keep_inner_face: true,
                        scale: 0.2,
                        mode: InsetScaleMode::default(),
                    })
                }
            } else if ui.button("Disable").clicked() {
                opts.insetting = None;
            }
        });
        ui.scope(|ui| {
            if let Some(inset) = &mut opts.insetting {
                bevy_inspector::ui_for_value(inset, ui, world);
            }
        });
    });
}

fn show_ui(world: &mut World) {
    let mut regenerate = false;
    world.resource_scope(|world, mut params: Mut<BuilderParams>| {
        let Ok(egui_context) = world.query::<&mut EguiContext>().get_single(world) else {
            return;
        };
        let mut egui_context = egui_context.clone();
        egui::SidePanel::left("Mesh settings").show(egui_context.get_mut(), |ui| {
            ui.heading("Global");
            egui::Grid::new("Grid").num_columns(2).show(ui, |ui| {
                ui.label("Range");
                ui.add(egui::DragValue::new(&mut params.range).range(1..=10));
                ui.end_row();
                ui.label("Max Height");
                ui.add(egui::DragValue::new(&mut params.max_height).range(1.0..=50.0));
                ui.end_row();
                ui.label("Scale");
                bevy_inspector::ui_for_value(&mut params.scale, ui, world);
                ui.end_row();
                ui.label("Fill Holes");
                ui.add(egui::Checkbox::without_text(&mut params.fill_holes));
                ui.end_row();
            });
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Caps Options");
                ui.push_id("Caps", |ui| {
                    face_opts(ui, &mut params.caps_options, world);
                });
                ui.separator();
                ui.heading("Sides Options");
                ui.push_id("Sides", |ui| {
                    face_opts(ui, &mut params.sides_options, world);
                });
            });
            ui.add_space(10.0);
            ui.vertical_centered_justified(|ui| {
                regenerate = ui.button("Generate").clicked();
            });
        });
    });
    world.resource_scope(|world, mut materials: Mut<Assets<StandardMaterial>>| {
        let Ok(egui_context) = world.query::<&mut EguiContext>().get_single(world) else {
            return;
        };
        let mut egui_context = egui_context.clone();
        let ctx = egui_context.get_mut();
        let rect = ctx.screen_rect().with_min_x(250.0);
        egui::Window::new("Visuals")
            .constrain_to(rect)
            .show(ctx, |ui| {
                ui.collapsing("Ambient Light", |ui| {
                    bevy_inspector::ui_for_resource::<AmbientLight>(world, ui);
                });
                let info = world.resource::<HexInfo>();
                let mat = materials.get_mut(&info.material_handle).unwrap();
                ui.collapsing("Material", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Base Color");
                        bevy_inspector::ui_for_value(&mut mat.base_color, ui, world);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Double sided");
                        bevy_inspector::ui_for_value(&mut mat.double_sided, ui, world);
                    });
                    match &mut mat.cull_mode {
                        Some(_) => {
                            if ui.button("No Culling").clicked() {
                                mat.cull_mode = None
                            }
                        }
                        None => {
                            if ui.button("Cull back faces").clicked() {
                                mat.cull_mode = Some(bevy::render::render_resource::Face::Back);
                            }
                        }
                    }
                });
            });
    });

    if regenerate {
        world.run_system_once(generate).unwrap();
    }
}

/// 3D Orthogrpahic camera setup
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("uv_checker.png");
    let transform = Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn((Camera3d::default(), transform));
    let transform = Transform::from_xyz(20.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn((DirectionalLight::default(), transform));
    let layout = HexLayout::default();
    let mesh = HeightMapMeshBuilder::new(&layout, &HashMap::new()).build();
    let mesh_handle = meshes.add(compute_mesh(mesh));
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture),
        cull_mode: None,
        double_sided: true,
        ..default()
    });
    let mesh_entity = commands
        .spawn((
            Mesh3d(mesh_handle.clone()),
            MeshMaterial3d(material_handle.clone()),
        ))
        .id();
    commands.insert_resource(HexInfo {
        layout,
        mesh_entity,
        mesh_handle,
        material_handle,
    });
}

fn animate(
    info: Res<HexInfo>,
    mut transforms: Query<&mut Transform>,
    mut motion_evr: EventReader<MouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
) {
    for event in motion_evr.read() {
        if buttons.pressed(MouseButton::Left) {
            let mut transform = transforms.get_mut(info.mesh_entity).unwrap();
            let axis = Vec3::new(event.delta.y, event.delta.x, 0.0).normalize();
            let angle = event.delta.length() * time.delta_secs();
            let quaternion = Quat::from_axis_angle(axis, angle);
            transform.rotate(quaternion);
        }
    }
}

fn gizmos(
    mut draw: Gizmos,
    info: Res<HexInfo>,
    transforms: Query<&Transform>,
    params: Res<BuilderParams>,
) {
    let transform = transforms.get(info.mesh_entity).unwrap();
    // Global axis
    // draw.axes(Transform::default(), 100.0);
    // Local axis
    let mut transform = *transform;
    transform.scale.y += params.max_height / 2.0;
    transform.scale.x += info.layout.scale.x;
    transform.scale.z += info.layout.scale.y;
    transform.scale *= params.scale;
    draw.axes(transform, 1.0);
}

fn generate(params: Res<BuilderParams>, info: Res<HexInfo>, mut meshes: ResMut<Assets<Mesh>>) {
    let mut rng = thread_rng();
    let map = HexagonalMap::new(Hex::ZERO, params.range, |_| {
        rng.gen_range(0.0..=params.max_height)
    });
    let mut new_mesh = HeightMapMeshBuilder::new(&info.layout, &map)
        .with_offset(Vec3::NEG_Y * params.max_height / 2.0 * params.scale.y)
        .with_scale(params.scale)
        .with_height_range(0.0..=params.max_height)
        .without_top_face()
        .without_sides();
    if let Some(opts) = params.caps_options {
        new_mesh = new_mesh.with_cap_options(opts);
    }
    if let Some(opts) = params.sides_options {
        new_mesh = new_mesh.with_side_options(opts);
    }
    if params.fill_holes {
        new_mesh = new_mesh.with_default_height(0.0);
    }
    let new_mesh = compute_mesh(new_mesh.build());
    // println!("Mesh has {} vertices", new_mesh.count_vertices());
    let mesh = meshes.get_mut(&info.mesh_handle).unwrap();
    *mesh = new_mesh;
}

/// Compute a bevy mesh from the layout
fn compute_mesh(mesh_info: MeshInfo) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}

impl Default for BuilderParams {
    fn default() -> Self {
        Self {
            max_height: 3.0,
            range: 3,
            sides_options: Some(FaceOptions::new()),
            caps_options: Some(FaceOptions::new()),
            scale: Vec3::ONE,
            fill_holes: false,
        }
    }
}
