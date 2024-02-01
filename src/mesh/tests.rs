use crate::{ColumnMeshBuilder, HexLayout, MeshInfo, PlaneMeshBuilder};

fn mesh_integrity(mesh: MeshInfo, expected_len: usize) {
    assert_eq!(mesh.vertices.len(), expected_len);
    assert_eq!(mesh.normals.len(), expected_len);
    assert_eq!(mesh.uvs.len(), expected_len);
    for normal in mesh.normals {
        assert!(normal.is_normalized());
    }
    for uv in mesh.uvs {
        assert!(uv.x >= 0.0);
        assert!(uv.y >= 0.0);
        assert!(uv.x <= 1.0);
        assert!(uv.y <= 1.0);
    }
}

#[test]
fn plane_integrity() {
    let mesh = PlaneMeshBuilder::new(&HexLayout::default()).build();
    mesh_integrity(mesh, 6);
}

#[test]
fn column_integrity() {
    let layout = HexLayout::default();
    let mesh = ColumnMeshBuilder::new(&layout, 10.0)
        .without_top_face()
        .without_bottom_face()
        .build();
    mesh_integrity(mesh, 6 * 4);
    let mesh = ColumnMeshBuilder::new(&layout, 10.0)
        .without_bottom_face()
        .build();
    mesh_integrity(mesh, 6 * 4 + 6);
    let mesh = ColumnMeshBuilder::new(&layout, 10.0).build();
    mesh_integrity(mesh, 6 * 4 + 12);
}
