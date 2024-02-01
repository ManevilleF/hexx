use bevy::{
    input::mouse::MouseMotion,
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::bevy_inspector;
use glam::vec2;
use hexx::*;

#[derive(Debug, Resource)]
struct HexInfo {
    pub layout: HexLayout,
    pub mesh_entity: Entity,
    pub mesh_handle: Handle<Mesh>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum SideUVMode {
    Global,
    Multi,
}

impl SideUVMode {
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
    pub sides_uvs_mode: SideUVMode,
    pub sides_uvs: [UVOptions; 6],
    pub caps_uvs: UVOptions,
}

pub fn main() {
    App::new()
        .init_resource::<BuilderParams>()
        .insert_resource(AmbientLight {
            brightness: 0.3,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(WireframePlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (show_ui, animate, update_mesh, gizmos))
        .run();
}

fn show_ui(world: &mut World) {
    world.resource_scope(|world, mut params: Mut<BuilderParams>| {
        let Ok(egui_context) = world.query::<&mut EguiContext>().get_single(world) else {
            return;
        };
        let mut egui_context = egui_context.clone();
        egui::SidePanel::left("Mesh settings").show(egui_context.get_mut(), |ui| {
            ui.heading("Global");
            egui::Grid::new("Grid").num_columns(2).show(ui, |ui| {
                ui.label("Column Height");
                ui.add(egui::DragValue::new(&mut params.height).clamp_range(1.0..=50.0));
                ui.end_row();
                ui.label("Side Subdivisions");
                ui.add(egui::DragValue::new(&mut params.subdivisions).clamp_range(0..=50));
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
            ui.heading("Caps UV options");
            bevy_inspector::ui_for_value(&mut params.caps_uvs, ui, world);
            ui.separator();
            ui.heading("Sides UV options");
            ui.horizontal(|ui| {
                egui::ComboBox::from_id_source("Side Uv mode")
                    .selected_text(params.sides_uvs_mode.label())
                    .show_ui(ui, |ui| {
                        let option = SideUVMode::Global;
                        ui.selectable_value(&mut params.sides_uvs_mode, option, option.label());
                        let option = SideUVMode::Multi;
                        ui.selectable_value(&mut params.sides_uvs_mode, option, option.label());
                    })
            });
            egui::ScrollArea::vertical().show(ui, |ui| match params.sides_uvs_mode {
                SideUVMode::Global => {
                    if bevy_inspector::ui_for_value(&mut params.sides_uvs[0], ui, world) {
                        params.sides_uvs = [params.sides_uvs[0]; 6];
                    }
                    true
                }
                SideUVMode::Multi => bevy_inspector::ui_for_value(&mut params.sides_uvs, ui, world),
            });
        });
        egui::Window::new("AmbientLight").show(egui_context.get_mut(), |ui| {
            bevy_inspector::ui_for_resource::<AmbientLight>(world, ui);
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
    commands.spawn(Camera3dBundle {
        transform,
        ..default()
    });
    let transform = Transform::from_xyz(20.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn(DirectionalLightBundle {
        transform,
        ..default()
    });
    let layout = HexLayout::default();
    let mesh = ColumnMeshBuilder::new(&layout, params.height)
        .with_subdivisions(params.subdivisions)
        .with_offset(Vec3::NEG_Y * params.height / 2.0)
        .build();
    let mesh_handle = meshes.add(compute_mesh(mesh));
    let material = materials.add(texture.into());
    let mesh_entity = commands
        .spawn((
            PbrBundle {
                mesh: mesh_handle.clone(),
                material,
                ..default()
            },
            Wireframe,
        ))
        .id();
    commands.insert_resource(HexInfo {
        layout,
        mesh_entity,
        mesh_handle,
    });
}

fn animate(
    info: Res<HexInfo>,
    mut transforms: Query<&mut Transform>,
    mut motion_evr: EventReader<MouseMotion>,
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
) {
    for event in motion_evr.read() {
        if buttons.pressed(MouseButton::Left) {
            let mut transform = transforms.get_mut(info.mesh_entity).unwrap();
            let axis = Vec3::new(event.delta.y, event.delta.x, 0.0).normalize();
            let angle = event.delta.length() * time.delta_seconds();
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
    draw.line(Vec3::NEG_X * 100.0, Vec3::X * 100.0, Color::RED.with_a(0.4));
    draw.line(
        Vec3::NEG_Y * 100.0,
        Vec3::Y * 100.0,
        Color::GREEN.with_a(0.4),
    );
    draw.line(
        Vec3::NEG_Z * 100.0,
        Vec3::Z * 100.0,
        Color::BLUE.with_a(0.4),
    );
    // Local axis
    let radius = info.layout.hex_size.length() * params.scale.length();
    draw.circle(Vec3::ZERO, transform.local_x(), radius, Color::RED)
        .segments(64);
    draw.circle(Vec3::ZERO, transform.local_y(), radius, Color::GREEN)
        .segments(64);
    draw.circle(Vec3::ZERO, transform.forward(), radius, Color::BLUE)
        .segments(64);
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
        .with_multi_sides_uv_options(match params.sides_uvs_mode {
            SideUVMode::Global => [params.sides_uvs[0]; 6],
            SideUVMode::Multi => params.sides_uvs,
        });
    if !params.top_face {
        new_mesh = new_mesh.without_top_face();
    }
    if !params.bottom_face {
        new_mesh = new_mesh.without_bottom_face();
    }
    let new_mesh = compute_mesh(new_mesh.build());
    // println!("Mesh has {} vertices", new_mesh.count_vertices());
    let mesh = meshes.get_mut(&info.mesh_handle).unwrap();
    *mesh = new_mesh;
}

/// Compute a bevy mesh from the layout
fn compute_mesh(mesh_info: MeshInfo) -> Mesh {
    Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
        .with_indices(Some(Indices::U16(mesh_info.indices)))
}

impl Default for BuilderParams {
    fn default() -> Self {
        Self {
            height: 10.0,
            subdivisions: 3,
            top_face: true,
            bottom_face: true,
            sides_uvs_mode: SideUVMode::Global,
            sides_uvs: [UVOptions::new().with_scale_factor(vec2(0.3, 1.0)); 6],
            caps_uvs: UVOptions::new(),
            scale: Vec3::ONE,
        }
    }
}
