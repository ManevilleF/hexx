use crate::{hex::ExactSizeHexIterator, Hex};

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

/// Generates an hexagonal layout around `center` with a custom `radius`.
#[must_use]
pub fn hexagon(center: Hex, radius: u32) -> impl ExactSizeIterator<Item = Hex> {
    center.range(radius)
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
