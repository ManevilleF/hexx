use crate::{DiagonalDirection, Direction, Hex};
use glam::{IVec2, IVec3, Vec2};

impl From<(i32, i32)> for Hex {
    #[inline]
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<[i32; 2]> for Hex {
    #[inline]
    fn from([x, y]: [i32; 2]) -> Self {
        Self { x, y }
    }
}

impl From<(f32, f32)> for Hex {
    #[inline]
    fn from(v: (f32, f32)) -> Self {
        Self::round(v)
    }
}

impl From<[f32; 2]> for Hex {
    #[inline]
    fn from([x, y]: [f32; 2]) -> Self {
        Self::round((x, y))
    }
}

impl From<Hex> for IVec2 {
    #[inline]
    fn from(hex: Hex) -> Self {
        hex.as_ivec2()
    }
}

impl From<Vec2> for Hex {
    #[inline]
    fn from(value: Vec2) -> Self {
        Self::round((value.x, value.y))
    }
}

impl From<Hex> for IVec3 {
    #[inline]
    fn from(hex: Hex) -> Self {
        hex.as_ivec3()
    }
}

impl From<IVec2> for Hex {
    #[inline]
    fn from(v: IVec2) -> Self {
        Self::new(v.x, v.y)
    }
}

impl From<Direction> for Hex {
    fn from(value: Direction) -> Self {
        Self::neighbor_coord(value)
    }
}

impl From<DiagonalDirection> for Hex {
    fn from(value: DiagonalDirection) -> Self {
        Self::diagonal_neighbor_coord(value)
    }
}
