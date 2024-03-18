#![allow(
    clippy::enum_glob_use,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss
)]

use super::*;
use crate::HexOrientation;
use std::f32::{consts::PI, EPSILON};

mod hex_directions {
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
            assert!(dir.angle_flat_degrees() - angle <= EPSILON);
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
            let expect = expected(angle);
            assert_eq!(
                EdgeDirection::from_flat_angle_degrees(angle).index(),
                expect
            );
            assert_eq!(EdgeDirection::from_flat_angle(angle_rad).index(), expect);
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
        let orientation = HexOrientation::Flat;
        for (dir, angle) in expected {
            assert!(dir.angle_flat() - angle <= EPSILON);
            assert!(dir.angle(orientation) - angle <= EPSILON);
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
            assert!(dir.angle_pointy_degrees() - angle <= EPSILON);
        }
    }

    #[test]
    fn from_pointy_angles() {
        let expected = |angle: f32| {
            let angle = angle % 360.0;
            let angle = if angle < 0.0 { angle + 360.0 } else { angle };
            match angle {
                v if v < 30.0 => EdgeDirection::FLAT_TOP_RIGHT,
                v if v < 90.0 => EdgeDirection::FLAT_TOP,
                v if v < 150.0 => EdgeDirection::FLAT_TOP_LEFT,
                v if v < 210.0 => EdgeDirection::FLAT_BOTTOM_LEFT,
                v if v < 270.0 => EdgeDirection::FLAT_BOTTOM,
                v if v < 330.0 => EdgeDirection::FLAT_BOTTOM_RIGHT,
                _ => EdgeDirection::FLAT_TOP_RIGHT,
            }
        };
        for angle in -1000..1000 {
            let angle = angle as f32 + 0.1;
            let angle_rad = angle.to_radians();
            let expect = expected(angle);
            assert_eq!(EdgeDirection::from_pointy_angle_degrees(angle), expect);
            assert_eq!(EdgeDirection::from_pointy_angle(angle_rad), expect);
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
        let orientation = HexOrientation::Pointy;
        for (dir, angle) in expected {
            assert!(dir.angle_pointy() - angle <= EPSILON);
            assert!(dir.angle(orientation) - angle <= EPSILON);
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

mod diagonal_direction {
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
            let expected = dir.direction_ccw().angle_flat() - PI / 6.0;
            assert!(dir.angle_flat() - expected <= EPSILON);
        }
    }

    #[test]
    fn pointy_angles_rad() {
        for dir in VertexDirection::ALL_DIRECTIONS {
            let expected = dir.direction_ccw().angle_pointy() - PI / 6.0;
            assert!(dir.angle_pointy() - expected <= EPSILON);
        }
    }

    #[test]
    fn flat_angles_degrees() {
        for dir in VertexDirection::ALL_DIRECTIONS {
            let expected = dir.direction_ccw().angle_flat_degrees() - 30.0;
            assert!(dir.angle_flat_degrees() - expected <= EPSILON);
        }
    }

    #[test]
    fn pointy_angles_degrees() {
        for dir in VertexDirection::ALL_DIRECTIONS {
            let expected = dir.direction_ccw().angle_pointy_degrees() - 30.0;
            assert!(dir.angle_pointy_degrees() - expected <= EPSILON);
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
            print!("pos angle = {angle}");
            match angle {
                v if v < 60.0 => TopRight,
                v if v < 120.0 => TopLeft,
                v if v < 180.0 => Left,
                v if v < 240.0 => BottomLeft,
                v if v < 300.0 => BottomRight,
                _ => Right,
            }
        };
        for angle in -1000..1000 {
            let angle = angle as f32 + 0.1;
            let angle_rad = angle.to_radians();
            let expect = expected(angle);
            println!("expect = {expect:?}, angle = {angle} ");
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
                v if v < 30.0 => Right,
                v if v < 90.0 => TopRight,
                v if v < 150.0 => TopLeft,
                v if v < 210.0 => Left,
                v if v < 270.0 => BottomLeft,
                v if v < 330.0 => BottomRight,
                _ => Right,
            }
        };
        for angle in -1000..1000 {
            let angle = angle as f32 + 0.1;
            let angle_rad = angle.to_radians();
            let expect = expected(angle);
            assert_eq!(VertexDirection::from_flat_angle_degrees(angle), expect);
            assert_eq!(VertexDirection::from_flat_angle(angle_rad), expect);
        }
    }
}
