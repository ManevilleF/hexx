use crate::Hex;
use std::collections::{HashMap, HashSet};

/// Computes a field of movement around `coord` given a `budget`.
/// This algorithm takes a `cost` function, which calculates and
/// returns the cost of movement through a given `Hex` tile.
/// The `cost` function should return an Option<u32>.
/// A tile that returns a computable cost would return Some(cost), whereas
/// None should be returned for tiles that have no computable cost (i.e. cannot be moved through).
///
/// # Examples
///
/// - Compute field of movement with no boundaries and some wall tiles that cannot be moved through
///
/// ```rust
/// # use hexx::*;
/// # use std::collections::HashSet;
/// use hexx::algorithms::field_of_movement;
///
/// let pos = hex(0, 0);
/// let range = 10;
/// let blocking_coords: HashSet<Hex> = HashSet::new();
/// // Add blocking coordinates
/// // blocking_coords.insert(hex(2, 0));
/// // ..
/// let fom = field_of_movement(pos, range, |h| if blocking_coords.contains(&h) {None} else {Some(1)});
/// ```
pub fn field_of_movement(
    coord: Hex,
    budget: u32,
    cost: impl Fn(Hex) -> Option<u32>,
) -> HashSet<Hex> {
    let mut computed_costs = HashMap::new();
    computed_costs.insert(coord, 0);
    let mut res = HashSet::new();
    for i in 1..=budget {
        for coord in coord.ring(i) {
            let coord_cost = if let Some(v) = cost(coord) {
                v + 1
            } else {
                continue;
            };

            let neighbor_cost = if let Some(cost) = coord
                .all_neighbors()
                .iter()
                .filter_map(|n| computed_costs.get(n))
                .min()
            {
                cost
            } else {
                continue;
            };

            let computed_cost = coord_cost + neighbor_cost;
            if computed_cost > budget {
                continue;
            }
            computed_costs.insert(coord, computed_cost);
            res.insert(coord);
        }
    }
    res
}
