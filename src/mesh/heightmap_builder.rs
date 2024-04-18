use std::collections::HashMap;

use glam::{Quat, Vec3};

use super::{utils::Quad, MeshInfo, BASE_FACING};
use crate::{EdgeDirection, Hex, HexLayout, InsetOptions, PlaneMeshBuilder, UVOptions};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct HeightMapMeshBuilder<'l, 'm> {
    /// The hexagonal layout, used to compute vertex positions
    pub layout: &'l HexLayout,
    /// The column height on missing neighbor
    pub base_height: Option<f32>,
    pub map: &'m HashMap<Hex, f32>,
}

impl<'l, 'm> HeightMapMeshBuilder<'l, 'm> {
    pub const fn new(layout: &'l HexLayout, map: &'m HashMap<Hex, f32>) -> Self {
        Self {
            layout,
            map,
            base_height: None,
        }
    }

    pub fn build(self) -> MeshInfo {
        // We create the final mesh
        let mut mesh = MeshInfo::default();

        for (&hex, &height) in self.map {
            let plane = PlaneMeshBuilder::new(self.layout)
                .at(hex)
                .with_offset(Vec3::Y * height)
                .build();
            mesh.merge_with(plane);
            let corners = self.layout.hex_edge_corners(hex);
            let dir_heights = EdgeDirection::ALL_DIRECTIONS.map(|dir| {
                (
                    dir,
                    self.map.get(&(hex + dir)).copied().or(self.base_height),
                )
            });
            for (dir, opt_height) in dir_heights {
                let Some(other_height) = opt_height else {
                    continue;
                };
                if other_height < height {
                    continue;
                }
                let points = corners[dir.index() as usize];
                let quad = Quad::from_bottom2(points, height, other_height);
                mesh.merge_with(quad.into());
            }
        }
        mesh
    }
}
