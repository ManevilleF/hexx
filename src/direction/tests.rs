#![allow(
    clippy::enum_glob_use,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss
)]

use approx::assert_relative_eq;

use super::*;
use crate::HexOrientation;
use std::f32::consts::PI;

mod edge_directions {
    use super::*;

    #[test]
    fn rotate_ccw_cw() {
        for direction in EdgeDirection::ALL_DIRECTIONS {
            assert_eq!(direction, direction.rotate_cw(6));
            assert_eq!(direction, direction.rotate_cw(12));
            assert_eq!(direction, direction.rotate_cw(1).rotate_ccw(1));
            assert_eq!(direction, direction.rotate_ccw(1).rotate_cw(1));
            assert_eq!(direction.counter_clockwise(), direction.rotate_ccw(1));
            assert_eq!(
                direction.counter_clockwise().counter_clockwise(),
                direction.rotate_ccw(2)
            );
            assert_eq!(direction.clockwise(), direction.rotate_cw(1));
            assert_eq!(direction.clockwise().clockwise(), direction.rotate_cw(2));
        }
    }

    #[test]
    fn rotations_reverse_each_other() {
        for direction in EdgeDirection::ALL_DIRECTIONS {
            assert_eq!(direction, direction.counter_clockwise().clockwise());
            assert_eq!(direction, direction.clockwise().counter_clockwise());
        }
    }

    #[test]
    fn six_rotations_comes_home() {
        for direction in EdgeDirection::ALL_DIRECTIONS {
            let mut clockwise_dir = direction;
            let mut counter_clockwise_dir = direction;

            for _ in 0..6 {
                clockwise_dir = clockwise_dir.counter_clockwise();
                counter_clockwise_dir = counter_clockwise_dir.clockwise();
            }

            assert_eq!(direction, clockwise_dir);
            assert_eq!(direction, counter_clockwise_dir);
        }
    }

    #[test]
    fn flat_angles_degrees() {
        let expected = [
            (EdgeDirection::FLAT_TOP_RIGHT, 30.0),
            (EdgeDirection::FLAT_TOP, 90.0),
            (EdgeDirection::FLAT_TOP_LEFT, 150.0),
            (EdgeDirection::FLAT_BOTTOM_LEFT, 210.0),
            (EdgeDirection::FLAT_BOTTOM, 270.0),
            (EdgeDirection::FLAT_BOTTOM_RIGHT, 330.0),
        ];
        for (dir, angle) in expected {
            assert_relative_eq!(dir.angle_flat_degrees(), angle);
        }
    }

    #[test]
    fn from_flat_angles() {
        let expected = |angle: f32| {
            let angle = angle % 360.0;
            let angle = if angle < 0.0 { angle + 360.0 } else { angle };
            match angle {
                v if v < 60.0 => 0,
                v if v < 120.0 => 1,
                v if v < 180.0 => 2,
                v if v < 240.0 => 3,
                v if v < 300.0 => 4,
                _ => 5,
            }
        };
        for angle in -1000..1000 {
            let angle = angle as f32 + 0.1;
            let angle_rad = angle.to_radians();
            let expect = EdgeDirection(expected(angle));
            assert_eq!(EdgeDirection::from_flat_angle_degrees(angle), expect);
            assert_eq!(EdgeDirection::from_flat_angle(angle_rad), expect);
        }
    }

    #[test]
    fn from_pointy_angles() {
        let expected = |angle: f32| {
            let angle = angle % 360.0;
            let angle = if angle < 0.0 { angle + 360.0 } else { angle };
            match angle {
                v if v < 30.0 => 0,
                v if v < 90.0 => 1,
                v if v < 150.0 => 2,
                v if v < 210.0 => 3,
                v if v < 270.0 => 4,
                v if v < 330.0 => 5,
                _ => 0,
            }
        };
        for angle in -1000..1000 {
            let angle = angle as f32 + 0.1;
            let angle_rad = angle.to_radians();
            let expect = EdgeDirection(expected(angle));
            assert_eq!(EdgeDirection::from_pointy_angle_degrees(angle), expect);
            assert_eq!(EdgeDirection::from_pointy_angle(angle_rad), expect);
        }
    }

    #[test]
    fn direction_angle_from_to() {
        for dir in EdgeDirection::ALL_DIRECTIONS {
            // degs
            let flat_angle = dir.angle_flat_degrees();
            assert_eq!(EdgeDirection::from_flat_angle_degrees(flat_angle), dir);
            let pointy_angle = dir.angle_pointy_degrees();
            assert_eq!(EdgeDirection::from_pointy_angle_degrees(pointy_angle), dir);
            // rads
            let flat_angle = dir.angle_flat();
            assert_eq!(EdgeDirection::from_flat_angle(flat_angle), dir);
            let pointy_angle = dir.angle_pointy();
            assert_eq!(EdgeDirection::from_pointy_angle(pointy_angle), dir);
        }
    }

    #[test]
    fn flat_angles_rad() {
        let expected = [
            (EdgeDirection::FLAT_TOP_RIGHT, PI / 6.0),
            (EdgeDirection::FLAT_TOP, PI / 2.0),
            (EdgeDirection::FLAT_TOP_LEFT, 5.0 * PI / 6.0),
            (EdgeDirection::FLAT_BOTTOM_LEFT, 7.0 * PI / 6.0),
            (EdgeDirection::FLAT_BOTTOM, 3.0 * PI / 2.0),
            (EdgeDirection::FLAT_BOTTOM_RIGHT, 11.0 * PI / 6.0),
        ];
        for (dir, angle) in expected {
            assert_relative_eq!(dir.angle_flat(), angle);
            assert_relative_eq!(dir.angle(HexOrientation::Flat), angle);
        }
    }

    #[test]
    fn pointy_angles_degrees() {
        let expected = [
            (EdgeDirection::FLAT_TOP_RIGHT, 0.0),
            (EdgeDirection::FLAT_TOP, 60.0),
            (EdgeDirection::FLAT_TOP_LEFT, 120.0),
            (EdgeDirection::FLAT_BOTTOM_LEFT, 180.0),
            (EdgeDirection::FLAT_BOTTOM, 240.0),
            (EdgeDirection::FLAT_BOTTOM_RIGHT, 300.0),
        ];
        for (dir, angle) in expected {
            assert_relative_eq!(dir.angle_pointy_degrees(), angle);
            assert_relative_eq!(dir.angle_degrees(HexOrientation::Pointy), angle);
        }
    }

    #[test]
    fn pointy_angles_rad() {
        let expected = [
            (EdgeDirection::FLAT_TOP_RIGHT, 0.0),
            (EdgeDirection::FLAT_TOP, PI / 3.0),
            (EdgeDirection::FLAT_TOP_LEFT, 2.0 * PI / 3.0),
            (EdgeDirection::FLAT_BOTTOM_LEFT, PI),
            (EdgeDirection::FLAT_BOTTOM, 4.0 * PI / 3.0),
            (EdgeDirection::FLAT_BOTTOM_RIGHT, 5.0 * PI / 3.0),
        ];
        for (dir, angle) in expected {
            assert_relative_eq!(dir.angle_pointy(), angle);
            assert_relative_eq!(dir.angle(HexOrientation::Pointy), angle);
        }
    }

    #[test]
    fn diagonal_neighbors() {
        for dir in EdgeDirection::ALL_DIRECTIONS {
            assert_eq!(dir.diagonal_ccw().direction_cw(), dir);
            assert_eq!(dir.diagonal_cw().direction_ccw(), dir);
        }
    }
}

mod vertex_direction {
    use super::*;

    #[test]
    fn dir_neighbors() {
        for dir in VertexDirection::ALL_DIRECTIONS {
            assert_eq!(dir.direction_ccw().diagonal_cw(), dir);
            assert_eq!(dir.direction_cw().diagonal_ccw(), dir);
        }
    }

    #[test]
    fn flat_angles_rad() {
        for dir in VertexDirection::ALL_DIRECTIONS {
            assert_relative_eq!(dir.angle_flat(), (f32::from(dir.index()) * PI / 3.0));
        }
        let expected = [
            (VertexDirection(0), 0.0),
            (VertexDirection(1), PI / 3.0),
            (VertexDirection(2), 2.0 * PI / 3.0),
            (VertexDirection(3), PI),
            (VertexDirection(4), 4.0 * PI / 3.0),
            (VertexDirection(5), 5.0 * PI / 3.0),
        ];
        for (dir, angle) in expected {
            assert_relative_eq!(dir.angle_flat(), angle);
            assert_relative_eq!(dir.angle(HexOrientation::Flat), angle);
        }
    }

    #[test]
    fn pointy_angles_rad() {
        let expected = [
            (VertexDirection(1), PI / 6.0),
            (VertexDirection(2), PI / 2.0),
            (VertexDirection(3), 5.0 * PI / 6.0),
            (VertexDirection(4), 7.0 * PI / 6.0),
            (VertexDirection(5), 3.0 * PI / 2.0),
            (VertexDirection(0), 11.0 * PI / 6.0),
        ];
        for (dir, angle) in expected {
            assert_relative_eq!(dir.angle_pointy(), angle);
            assert_relative_eq!(dir.angle(HexOrientation::Pointy), angle);
        }
    }

    #[test]
    fn flat_angles_degrees() {
        let expected = [
            (VertexDirection(0), 0.0),
            (VertexDirection(1), 60.0),
            (VertexDirection(2), 120.0),
            (VertexDirection(3), 180.0),
            (VertexDirection(4), 240.0),
            (VertexDirection(5), 300.0),
        ];
        for (dir, angle) in expected {
            assert_relative_eq!(dir.angle_flat_degrees(), angle);
            assert_relative_eq!(dir.angle_degrees(HexOrientation::Flat), angle);
        }
    }

    #[test]
    fn pointy_angles_degrees() {
        let expected = [
            (VertexDirection(0), 330.0),
            (VertexDirection(1), 30.0),
            (VertexDirection(2), 90.0),
            (VertexDirection(3), 150.0),
            (VertexDirection(4), 210.0),
            (VertexDirection(5), 270.0),
        ];
        for (dir, angle) in expected {
            assert_relative_eq!(dir.angle_pointy_degrees(), angle);
            assert_relative_eq!(dir.angle_degrees(HexOrientation::Pointy), angle);
        }
    }

    #[test]
    fn direction_angle_from_to() {
        for dir in VertexDirection::ALL_DIRECTIONS {
            // degs
            let flat_angle = dir.angle_flat_degrees();
            assert_eq!(VertexDirection::from_flat_angle_degrees(flat_angle), dir);
            let pointy_angle = dir.angle_pointy_degrees();
            assert_eq!(
                VertexDirection::from_pointy_angle_degrees(pointy_angle),
                dir
            );
            // rads
            let flat_angle = dir.angle_flat();
            assert_eq!(VertexDirection::from_flat_angle(flat_angle), dir);
            let pointy_angle = dir.angle_pointy();
            assert_eq!(VertexDirection::from_pointy_angle(pointy_angle), dir);
        }
    }

    #[test]
    fn from_pointy_angles() {
        let expected = |angle: f32| {
            let angle = angle % 360.0;
            let angle = if angle < 0.0 { angle + 360.0 } else { angle };
            match angle {
                v if v < 60.0 => 1,
                v if v < 120.0 => 2,
                v if v < 180.0 => 3,
                v if v < 240.0 => 4,
                v if v < 300.0 => 5,
                _ => 0,
            }
        };
        for angle in -1000..1000 {
            let angle = angle as f32 + 0.1;
            let angle_rad = angle.to_radians();
            let expect = VertexDirection(expected(angle));
            assert_eq!(VertexDirection::from_pointy_angle_degrees(angle), expect);
            assert_eq!(VertexDirection::from_pointy_angle(angle_rad), expect);
        }
    }

    #[test]
    fn from_flat_angles() {
        let expected = |angle: f32| {
            let angle = angle % 360.0;
            let angle = if angle < 0.0 { angle + 360.0 } else { angle };
            match angle {
                v if v < 30.0 => 0,
                v if v < 90.0 => 1,
                v if v < 150.0 => 2,
                v if v < 210.0 => 3,
                v if v < 270.0 => 4,
                v if v < 330.0 => 5,
                _ => 0,
            }
        };
        for angle in -1000..1000 {
            let angle = angle as f32 + 0.1;
            let angle_rad = angle.to_radians();
            let expect = VertexDirection(expected(angle));
            assert_eq!(VertexDirection::from_flat_angle_degrees(angle), expect);
            assert_eq!(VertexDirection::from_flat_angle(angle_rad), expect);
        }
    }
}

#[test]
fn edge_vs_vertex() {
    for i in 0..6 {
        let edge = EdgeDirection(i);
        let vertex = VertexDirection(i);
        assert_relative_eq!(edge.angle_flat(), vertex.counter_clockwise().angle_pointy());
        assert_relative_eq!(edge.angle_pointy(), vertex.angle_flat());
        assert_relative_eq!(
            edge.angle_flat_degrees(),
            vertex.counter_clockwise().angle_pointy_degrees()
        );
        assert_relative_eq!(edge.angle_pointy_degrees(), vertex.angle_flat_degrees());
    }
}

#[test]
fn rad_deg() {
    for i in 0..6 {
        let edge = EdgeDirection(i);
        let vertex = VertexDirection(i);
        for orientation in [HexOrientation::Flat, HexOrientation::Pointy] {
            let rad = edge.angle(orientation);
            let degs = edge.angle_degrees(orientation);
            assert_relative_eq!(rad, degs.to_radians());
            assert_relative_eq!(degs, rad.to_degrees());

            let rad = vertex.angle(orientation);
            let degs = vertex.angle_degrees(orientation);
            assert_relative_eq!(rad, degs.to_radians());
            assert_relative_eq!(degs, rad.to_degrees());
        }
    }
}
