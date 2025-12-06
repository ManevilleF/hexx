use bevy::{
    asset::RenderAssetUsages,
    input::mouse::MouseMotion,
    mesh::Indices,
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
    render::render_resource::PrimitiveTopology,
};
use bevy_egui::{
    EguiContext, EguiPlugin,
    egui::{self, Ui},
};
use bevy_inspector_egui::bevy_inspector;
use hexx::*;

#[derive(Debug, Resource)]
struct HexInfo {
    pub layout: HexLayout,
    pub mesh_entity: Entity,
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Handle<StandardMaterial>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum SideEditMode {
    Global,
    Multi,
}

impl SideEditMode {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Global => "Global",
            Self::Multi => "Multi",
        }
    }
}

#[derive(Debug, Resource)]
struct BuilderParams {
    pub height: f32,
    pub subdivisions: usize,
    pub top_face: bool,
    pub bottom_face: bool,
    pub scale: Vec3,
    pub sides_edit_mode: SideEditMode,
    pub side_glob_options: FaceOptions,
    pub sides_options: [Option<FaceOptions>; 6],
    pub caps_uvs: UVOptions,
    pub caps_inset: Option<InsetOptions>,
}

pub fn main() {
    App::new()
        .init_resource::<BuilderParams>()
        .insert_resource(AmbientLight {
            brightness: 500.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(WireframePlugin::default())
        .add_plugins(EguiPlugin::default())
        .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (show_ui, animate, update_mesh, gizmos))
        .run();
}

fn show_ui(world: &mut World) {
    world.resource_scope(|world, mut params: Mut<BuilderParams>| {
        let Ok(egui_context) = world.query::<&mut EguiContext>().single(world) else {
            return;
        };
        let mut egui_context = egui_context.clone();
        egui::SidePanel::left("Mesh settings").show(egui_context.get_mut(), |ui| {
            ui.heading("Global");
            egui::Grid::new("Grid").num_columns(2).show(ui, |ui| {
                ui.label("Column Height");
                ui.add(egui::DragValue::new(&mut params.height).range(1.0..=50.0));
                ui.end_row();
                ui.label("Side Subdivisions");
                ui.add(egui::DragValue::new(&mut params.subdivisions).range(0..=50));
                ui.end_row();
                ui.label("Top Face");
                ui.add(egui::Checkbox::without_text(&mut params.top_face));
                ui.end_row();
                ui.label("Bottom Face");
                ui.add(egui::Checkbox::without_text(&mut params.bottom_face));
                ui.end_row();
                ui.label("Scale");
                bevy_inspector::ui_for_value(&mut params.scale, ui, world);
                ui.end_row();
            });
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.push_id("Caps", |ui| {
                    ui.heading("Caps UV options");
                    bevy_inspector::ui_for_value(&mut params.caps_uvs, ui, world);
                    ui.heading("Caps Inset options");
                    ui.scope(|ui| match &mut params.caps_inset {
                        Some(opts) => {
                            bevy_inspector::ui_for_value(opts, ui, world);
                            if ui.button("Disable").clicked() {
                                params.caps_inset = None;
                            }
                        }
                        None => {
                            if ui.button("Enable").clicked() {
                                params.caps_inset = Some(InsetOptions {
                                    keep_inner_face: true,
                                    scale: 0.2,
                                    mode: InsetScaleMode::default(),
                                })
                            }
                        }
                    });
                });
                ui.separator();
                ui.heading("Sides Options");
                ui.horizontal(|ui| {
                    egui::ComboBox::from_id_salt("Side Edit Mode")
                        .selected_text(params.sides_edit_mode.label())
                        .show_ui(ui, |ui| {
                            let option = SideEditMode::Global;
                            ui.selectable_value(
                                &mut params.sides_edit_mode,
                                option,
                                option.label(),
                            );
                            let option = SideEditMode::Multi;
                            ui.selectable_value(
                                &mut params.sides_edit_mode,
                                option,
                                option.label(),
                            );
                        })
                });

                let mut side_opts = |ui: &mut Ui, options: &mut FaceOptions| {
                    ui.scope(|ui| {
                        ui.label("UV");
                        bevy_inspector::ui_for_value(&mut options.uv, ui, world);
                        ui.horizontal(|ui| {
                            ui.label("Insetting");
                            if options.insetting.is_none() {
                                if ui.button("Enable").clicked() {
                                    options.insetting = Some(InsetOptions {
                                        keep_inner_face: true,
                                        scale: 0.2,
                                        mode: InsetScaleMode::default(),
                                    })
                                }
                            } else if ui.button("Disable").clicked() {
                                options.insetting = None;
                            }
                        });
                        ui.scope(|ui| {
                            if let Some(inset) = &mut options.insetting {
                                bevy_inspector::ui_for_value(inset, ui, world);
                            }
                        });
                    });
                };

                match params.sides_edit_mode {
                    SideEditMode::Global => {
                        side_opts(ui, &mut params.side_glob_options);
                    }
                    SideEditMode::Multi => {
                        for dir in EdgeDirection::ALL_DIRECTIONS {
                            let option = &mut params.sides_options[dir.index() as usize];
                            ui.add_space(10.0);
                            ui.strong(format!("{dir:?}"));
                            if option.is_none() {
                                if ui.button("Enable").clicked() {
                                    *option = Some(FaceOptions::new());
                                }
                            } else if ui.button("Disable").clicked() {
                                *option = None;
                            }
                            if let Some(opts) = option {
                                ui.push_id(dir, |ui| {
                                    side_opts(ui, opts);
                                });
                            }
                        }
                    }
                }
            });
        });
    });
    world.resource_scope(|world, mut materials: Mut<Assets<StandardMaterial>>| {
        let Ok(egui_context) = world.query::<&mut EguiContext>().single(world) else {
            return;
        };
        let mut egui_context = egui_context.clone();
        let ctx = egui_context.get_mut();
        let rect = ctx.content_rect().with_min_x(250.0);
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
}

/// 3D Orthogrpahic camera setup
fn setup(
    mut commands: Commands,
    params: Res<BuilderParams>,
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
    let mesh = ColumnMeshBuilder::new(&layout, params.height)
        .with_subdivisions(params.subdivisions)
        .with_offset(Vec3::NEG_Y * params.height / 2.0)
        .build();
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
            Wireframe,
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
    mut motion_evr: MessageReader<MouseMotion>,
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
    draw.axes(Transform::default(), 100.0);
    // Local axis
    let mut transform = *transform;
    transform.scale.y += params.height / 2.0;
    transform.scale.x += info.layout.scale.x;
    transform.scale.z += info.layout.scale.y;
    transform.scale *= params.scale;
    draw.axes(transform, 1.0);
}

fn update_mesh(params: Res<BuilderParams>, info: Res<HexInfo>, mut meshes: ResMut<Assets<Mesh>>) {
    if !params.is_changed() {
        return;
    }
    let mut new_mesh = ColumnMeshBuilder::new(&info.layout, params.height)
        .with_subdivisions(params.subdivisions)
        .with_offset(Vec3::NEG_Y * params.height / 2.0 * params.scale.y)
        .with_scale(params.scale)
        .with_caps_uv_options(params.caps_uvs)
        .with_multi_custom_sides_options(match params.sides_edit_mode {
            SideEditMode::Global => [Some(params.side_glob_options); 6],
            SideEditMode::Multi => params.sides_options,
        });
    if !params.top_face {
        new_mesh = new_mesh.without_top_face();
    }
    if !params.bottom_face {
        new_mesh = new_mesh.without_bottom_face();
    }
    if let Some(opts) = params.caps_inset {
        new_mesh = new_mesh.with_caps_inset_options(opts);
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
            height: 10.0,
            subdivisions: 3,
            top_face: true,
            bottom_face: true,
            sides_edit_mode: SideEditMode::Global,
            side_glob_options: FaceOptions::new(),
            sides_options: [Some(FaceOptions::new()); 6],
            caps_uvs: UVOptions::new(),
            scale: Vec3::ONE,
            caps_inset: None,
        }
    }
}
