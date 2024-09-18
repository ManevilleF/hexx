use super::{utils::Quad, MeshInfo};
use crate::{EdgeDirection, Hex, HexLayout, PlaneMeshBuilder, UVOptions};
use glam::Vec3;
use std::collections::HashMap;

pub struct HeightMapMeshBuilder<'l, 'm> {
    /// The hexagonal layout, used to compute vertex positions
    pub layout: &'l HexLayout,
    /// The column height on missing neighbor
    pub base_height: Option<f32>,
    pub map: &'m HashMap<Hex, f32>,

    pub caps_uv_options: Option<Box<dyn Fn(Hex, f32) -> UVOptions>>,
    pub sides_uv_options: Option<Box<dyn Fn(Hex, EdgeDirection, f32) -> UVOptions>>,
}

impl<'l, 'm> HeightMapMeshBuilder<'l, 'm> {
    pub const fn new(layout: &'l HexLayout, map: &'m HashMap<Hex, f32>) -> Self {
        Self {
            layout,
            map,
            base_height: None,
            caps_uv_options: None,
            sides_uv_options: None,
        }
    }

    pub fn build(self) -> MeshInfo {
        // We create the final mesh
        let mut mesh = MeshInfo::default();

        let min = self.map.values().copied().reduce(f32::min).unwrap_or(0.0);
        let max = self.map.values().copied().reduce(f32::max).unwrap_or(0.0);

        for (&hex, &height) in self.map {
            let mut plane = PlaneMeshBuilder::new(self.layout)
                .at(hex)
                .with_offset(Vec3::Y * height);
            if let Some(opt) = &self.caps_uv_options {
                let options = opt(hex, height);
                plane = plane.with_uv_options(options);
            }
            mesh.merge_with(plane.build());
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
                if other_height <= height {
                    continue;
                }
                let points = corners[dir.index() as usize];

                let quad = self.sides_uv_options.as_ref().map_or_else(
                    || Quad::new_bounded(points, height, other_height, [min, max]),
                    |opt| {
                        let mut quad = Quad::new(points, height, other_height);
                        let options = opt(hex, dir, other_height - height);
                        options.alter_uvs(&mut quad.uvs);
                        quad
                    },
                );
                mesh.merge_with(quad.into());
            }
        }
        mesh
    }
}
