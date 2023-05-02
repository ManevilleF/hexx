use bevy::{
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use hexx::*;

pub fn main() {
    App::new()
        .insert_resource(AmbientLight {
            brightness: 0.1,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_startup_system(setup)
        .add_system(animate)
        .run();
}

#[derive(Debug, Resource)]
struct HexInfo {
    pub mesh_entity: Entity,
}

/// 3D Orthogrpahic camera setup
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let transform = Transform::from_xyz(0.0, 0.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn(Camera3dBundle {
        transform,
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        transform,
        ..default()
    });
    let layout = HexLayout::default();
    let mesh = ColumnMeshBuilder::new(&layout, 15.0)
        .with_subdivisions(5)
        .build();
    let mesh = meshes.add(compute_mesh(mesh));
    let material = materials.add(Color::WHITE.into());
    let mesh_entity = commands
        .spawn((
            PbrBundle {
                mesh,
                material,
                ..default()
            },
            Wireframe,
        ))
        .id();
    commands.insert_resource(HexInfo { mesh_entity });
}

fn animate(info: Res<HexInfo>, mut transforms: Query<&mut Transform>, time: Res<Time>) {
    let delta_time = time.delta_seconds() / 2.0;
    let mut transform = transforms.get_mut(info.mesh_entity).unwrap();
    transform.rotate_x(delta_time);
    transform.rotate_y(delta_time);
    transform.rotate_local_y(delta_time);
    transform.rotate_z(delta_time);
}

/// Compute a bevy mesh from the layout
fn compute_mesh(mesh_info: MeshInfo) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs.to_vec());
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
}
