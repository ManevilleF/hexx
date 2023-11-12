use bevy::{
    input::mouse::MouseMotion,
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use bevy_inspector_egui::{prelude::*, quick::ResourceInspectorPlugin};
use glam::vec2;
use hexx::*;

#[derive(Debug, Resource)]
struct HexInfo {
    pub layout: HexLayout,
    pub mesh_entity: Entity,
    pub mesh_handle: Handle<Mesh>,
}

#[derive(Debug, Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct BuilderParams {
    #[inspector(min = 0.0, max = 50.0)]
    pub height: f32,
    #[inspector(min = 1, max = 50)]
    pub subdivisions: usize,
    pub top_face: bool,
    pub bottom_face: bool,
    pub sides_uvs: UVOptions,
    pub caps_uvs: UVOptions,
    pub scale: Vec3,
}

pub fn main() {
    App::new()
        .register_type::<BuilderParams>()
        .init_resource::<BuilderParams>()
        .insert_resource(AmbientLight {
            brightness: 0.3,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(WireframePlugin)
        .add_plugins(ResourceInspectorPlugin::<BuilderParams>::default())
        .add_plugins(ResourceInspectorPlugin::<AmbientLight>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (animate, update_mesh))
        .run();
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
    let transform = Transform::from_xyz(0.0, 0.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y);
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
    if buttons.pressed(MouseButton::Left) {
        for event in motion_evr.read() {
            let mut transform = transforms.get_mut(info.mesh_entity).unwrap();
            transform.rotate_y(event.delta.x * time.delta_seconds());
            transform.rotate_x(event.delta.y * time.delta_seconds());
        }
    }
}

fn update_mesh(params: Res<BuilderParams>, info: Res<HexInfo>, mut meshes: ResMut<Assets<Mesh>>) {
    if !params.is_changed() {
        return;
    }
    let mut new_mesh = ColumnMeshBuilder::new(&info.layout, params.height)
        .with_subdivisions(params.subdivisions)
        .with_offset(Vec3::NEG_Y * params.height / 2.0)
        .with_scale(params.scale)
        .with_caps_uv_options(params.caps_uvs.clone())
        .with_sides_uv_options(params.sides_uvs.clone());
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
            sides_uvs: UVOptions::quad_default().with_scale_factor(vec2(1.0, 0.3)),
            caps_uvs: UVOptions::cap_default().with_scale_factor(vec2(0.5, 0.5)),
            scale: Vec3::ONE,
        }
    }
}
