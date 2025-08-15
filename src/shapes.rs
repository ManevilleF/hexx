use crate::{Hex, hex::ExactSizeHexIterator};

/// Parallelogram shape parameters.
///
/// Calling `coords` will return coordinates in that shape.
/// Equivalent to [`parallelogram`]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Parallelogram {
    /// Start coordinate
    pub min: Hex,
    /// End coordinate
    pub max: Hex,
}

impl Default for Parallelogram {
    fn default() -> Self {
        Self {
            min: Hex::splat(-10),
            max: Hex::splat(10),
        }
    }
}

impl Parallelogram {
    /// Generates a [`parallelogram`] with the shape parameters
    #[must_use]
    pub fn coords(self) -> impl ExactSizeIterator<Item = Hex> {
        parallelogram(self.min, self.max)
    }
}

/// Generates a parallelogram layout from `min` to `max`
#[must_use]
#[allow(clippy::cast_sign_loss)]
pub fn parallelogram(min: Hex, max: Hex) -> impl ExactSizeIterator<Item = Hex> {
    let dist = (max.x.saturating_sub(min.x) + 1) * (max.y.saturating_sub(min.y) + 1);
    ExactSizeHexIterator {
        iter: (min.x()..=max.x())
            .flat_map(move |x| (min.y()..=max.y()).map(move |y| Hex::new(x, y))),
        count: dist as usize,
    }
}

/// Triangle shape parameters.
///
/// Calling `coords` will return coordinates in that shape.
/// Equivalent to [`triangle`]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Triangle {
    /// Triangle size
    pub size: u32,
}

impl Default for Triangle {
    fn default() -> Self {
        Self { size: 10 }
    }
}

impl Triangle {
    /// Generates a [`triangle`] with the shape parameters
    #[must_use]
    pub fn coords(self) -> impl ExactSizeIterator<Item = Hex> {
        triangle(self.size)
    }
}
/// Generates a triangle with a custom `size`
///
/// # Note
///
/// To offset the map, apply the offset to each `Item` of the returned iterator
#[allow(clippy::cast_possible_wrap)]
#[must_use]
pub fn triangle(size: u32) -> impl ExactSizeIterator<Item = Hex> {
    ExactSizeHexIterator {
        iter: (0..=size)
            .flat_map(move |x| (0..=(size - x)).map(move |y| Hex::new(x as i32, y as i32))),
        count: Hex::wedge_count(size) as usize,
    }
}

/// Hexagon shape parameters.
///
/// Calling `coords` will return coordinates in that shape.
/// Equivalent to [`hexagon`]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Hexagon {
    /// Center coordinate
    pub center: Hex,
    /// Hexagon radius
    pub radius: u32,
}

impl Default for Hexagon {
    fn default() -> Self {
        Self {
            center: Hex::ZERO,
            radius: 10,
        }
    }
}

impl Hexagon {
    /// Generates a [`hexagon`] with the shape parameters
    #[must_use]
    pub fn coords(self) -> impl ExactSizeIterator<Item = Hex> {
        hexagon(self.center, self.radius)
    }
}

/// Generates an hexagonal layout around `center` with a custom `radius`.
#[must_use]
pub fn hexagon(center: Hex, radius: u32) -> impl ExactSizeIterator<Item = Hex> {
    center.range(radius)
}

/// Rombus shape parameters.
///
/// Calling `coords` will return coordinates in that shape.
/// Equivalent to [`rombus`]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Rombus {
    /// rombus lowest coordinate
    pub origin: Hex,
    /// Row count (`y`)
    pub rows: u32,
    /// Column count (`x`)
    pub columns: u32,
}

impl Default for Rombus {
    fn default() -> Self {
        Self {
            origin: Hex::ZERO,
            rows: 10,
            columns: 10,
        }
    }
}

impl Rombus {
    /// Generates a [`rombus`] with the shape parameters
    #[must_use]
    pub fn coords(self) -> impl ExactSizeIterator<Item = Hex> {
        rombus(self.origin, self.rows, self.columns)
    }
}

/// Generates a Rombus from `point` of `rows` in y and `columns` in `x`
#[must_use]
#[allow(clippy::cast_possible_wrap)]
pub fn rombus(point: Hex, rows: u32, columns: u32) -> impl ExactSizeIterator<Item = Hex> {
    ExactSizeHexIterator {
        iter: (0..rows).flat_map(move |y| {
            (0..columns).map(move |x| point.const_add(Hex::new(x as i32, y as i32)))
        }),
        count: (rows * columns) as usize,
    }
}

/// [Pointy] rectangle shape parameters.
///
/// Calling `coords` will return coordinates in that shape.
/// Equivalent to [`pointy_rectangle`]
///
/// [Pointy]: crate::HexOrientation::Pointy
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct PointyRectangle {
    /// Lowest `x` coordinate
    pub left: i32,
    /// Highest `x` coordinate
    pub right: i32,
    /// Lowest `y` coordinate
    pub top: i32,
    /// Highest `y` coordinate
    pub bottom: i32,
}

impl Default for PointyRectangle {
    fn default() -> Self {
        Self {
            left: -10,
            right: 10,
            top: -10,
            bottom: 10,
        }
    }
}

impl PointyRectangle {
    /// Generates a [`pointy_rectangle`] with the shape parameters
    #[must_use]
    pub fn coords(self) -> impl ExactSizeIterator<Item = Hex> {
        pointy_rectangle([self.left, self.right, self.top, self.bottom])
    }
}

/// Generates a rectangle with the given bounds for "pointy topped" hexagons.
///
/// The function takes four offsets as `[left, right, top, bottom]`.
/// The generations goes `left` to `right` and `top` to `bottom`, therefore
/// `right` must be greater than `left` and `bottom` must be greater than `top`.
#[must_use]
#[allow(clippy::cast_sign_loss)]
pub fn pointy_rectangle(
    [left, right, top, bottom]: [i32; 4],
) -> impl ExactSizeIterator<Item = Hex> {
    let count = (right.saturating_sub(left) + 1) * (bottom.saturating_sub(top) + 1);
    ExactSizeHexIterator {
        iter: (top..=bottom).flat_map(move |y| {
            let y_offset = y >> 1;
            ((left - y_offset)..=(right - y_offset)).map(move |x| Hex::new(x, y))
        }),
        count: count as usize,
    }
}

/// [Flat] rectangle shape parameters.
///
/// Calling `coords` will return coordinates in that shape.
/// Equivalent to [`pointy_rectangle`]
///
/// [Flat]: crate::HexOrientation::Flat
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct FlatRectangle {
    /// Lowest `x` coordinate
    pub left: i32,
    /// Highest `x` coordinate
    pub right: i32,
    /// Lowest `y` coordinate
    pub top: i32,
    /// Highest `y` coordinate
    pub bottom: i32,
}

impl Default for FlatRectangle {
    fn default() -> Self {
        Self {
            left: -10,
            right: 10,
            top: -10,
            bottom: 10,
        }
    }
}

impl FlatRectangle {
    /// Generates a [`flat_rectangle`] with the shape parameters
    #[must_use]
    pub fn coords(self) -> impl ExactSizeIterator<Item = Hex> {
        flat_rectangle([self.left, self.right, self.top, self.bottom])
    }
}

/// Generates a rectangle with the given bounds for "flat topped" hexagons
///
/// The function takes four offsets as `[left, right, top, bottom]`.
/// The generations goes `left` to `right` and `top` to `bottom`, therefore
/// `right` must be greater than `left` and `bottom` must be greater than `top`.
#[must_use]
#[allow(clippy::cast_sign_loss)]
pub fn flat_rectangle([left, right, top, bottom]: [i32; 4]) -> impl ExactSizeIterator<Item = Hex> {
    let count = (right.saturating_sub(left) + 1) * (bottom.saturating_sub(top) + 1);
    ExactSizeHexIterator {
        iter: (left..=right).flat_map(move |x| {
            let x_offset = x >> 1;
            ((top - x_offset)..=(bottom - x_offset)).map(move |y| Hex::new(x, y))
        }),
        count: count as usize,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hexagon_test() {
        let point = Hex::new(3, -987);
        for range in 0..=30 {
            let iter = hexagon(point, range);
            assert_eq!(iter.len(), iter.count());
        }
    }

    #[test]
    fn triangle_test() {
        for range in 0..=30 {
            let iter = triangle(range);
            assert_eq!(iter.len(), iter.count());
        }
    }

    #[test]
    fn parallelogram_test() {
        for min in 0..=30 {
            for max in 0..=30 {
                let iter = parallelogram(Hex::splat(min), Hex::splat(min + max));
                assert_eq!(iter.len(), iter.count());
            }
        }
    }

    #[test]
    fn rombus_test() {
        for columns in 0..=30 {
            for rows in 0..=30 {
                for p in Hex::ZERO.range(10) {
                    let iter = rombus(p, rows, columns);
                    assert_eq!(iter.len(), iter.count());
                }
            }
        }
    }

    #[test]
    fn pointy_rectangle_test() {
        for left in -20..=20 {
            for right in 0..=20 {
                for top in -20..=20 {
                    for bottom in 0..=20 {
                        let iter = pointy_rectangle([left, left + right, top, top + bottom]);
                        assert_eq!(iter.len(), iter.count());
                    }
                }
            }
        }
    }

    #[test]
    fn flat_rectangle_test() {
        for left in -20..=20 {
            for right in 0..=20 {
                for top in -20..=20 {
                    for bottom in 0..=20 {
                        let iter = flat_rectangle([left, left + right, top, top + bottom]);
                        assert_eq!(iter.len(), iter.count());
                    }
                }
            }
        }
    }
}
