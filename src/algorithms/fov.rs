use crate::{EdgeDirection, Hex};
use std::collections::HashSet;

/// Computes a field of view around `coord` in a given `range`.
/// This algorithm takes in account coordinates *visibility* through the
/// `blocking` argument. (*Blocking* coordinates should return `true`)
///
/// # Examples
///
/// - Compute field of view with no boundaries and some blocking tiles
///
/// ```rust
/// # use hexx::*;
/// # use std::collections::HashSet;
/// use hexx::algorithms::range_fov;
///
/// let pos = hex(0, 0);
/// let range = 10;
/// let blocking_coords: HashSet<Hex> = HashSet::new();
/// // Add blocking coordinates
/// // blocking_coords.insert(hex(2, 0));
/// // ..
/// let fov = range_fov(pos, range, |h| blocking_coords.contains(&h));
/// ```
pub fn range_fov(coord: Hex, range: u32, blocking: impl Fn(Hex) -> bool) -> HashSet<Hex> {
    coord
        .ring(range)
        .flat_map(|target| coord.line_to(target).take_while(|h| !blocking(*h)))
        .collect()
}

/// Computes a field of view around `coord` in a given `range` towards
/// `direction` with 120 degrees vision.
/// This algorithm takes in account coordinates *visibility* through the
/// `blocking` argument. (*Blocking* coordinates should return `true`)
///
/// # Examples
///
/// - Compute drectional field of view with no boundaries and some blocking
///   tiles
///
/// ```rust
/// # use hexx::*;
/// # use std::collections::HashSet;
/// use hexx::algorithms::directional_fov;
///
/// let pos = hex(0, 0);
/// let range = 10;
/// let dir = Direction::Top;
/// let blocking_coords: HashSet<Hex> = HashSet::new();
/// // Add blocking coordinates
/// // blocking_coords.insert(hex(2, 0));
/// // ..
/// let fov = directional_fov(pos, range, dir, |h| blocking_coords.contains(&h));
/// ```
pub fn directional_fov(
    coord: Hex,
    range: u32,
    direction: EdgeDirection,
    blocking: impl Fn(Hex) -> bool,
) -> HashSet<Hex> {
    let [a, b] = direction.vertex_directions();
    coord
        .ring(range)
        .filter(|h| {
            let way = coord.diagonal_way_to(*h);
            way == a || way == b
        })
        .flat_map(|target| coord.line_to(target).take_while(|h| !blocking(*h)))
        .collect()
}
