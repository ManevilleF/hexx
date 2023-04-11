use crate::Hex;
use std::collections::{HashMap, HashSet};

/// Computes a field of movement around `coord` given a `budget`.
/// This algorithm takes a `cost` function, which calculates and
/// returns the cost of movement through a given `Hex` tile.
/// The `cost` function should return an `Option<u32>`.
/// A tile that returns a computable cost would return `Some(cost)`, whereas
/// `None` should be returned for tiles that have no computable cost (i.e. cannot be moved through).
///
/// The `field_of_movement` algorithm will always add `+ 1` to the computed cost in order to avoid
/// the possibility of unlimited movement range (i.e. a `Hex` instance will always have a minimum movement `cost` of 1).
///
/// # Examples
///
/// - Compute field of movement with no boundaries and some wall tiles that cannot be traversed
///
/// ```rust
/// # use hexx::*;
/// # use std::collections::HashMap;
/// use hexx::algorithms::field_of_movement;
///
/// enum Biome {
///    Mountain,
///    Plains,
///    Forest,
///    Desert
/// }
///
/// impl Biome {
///
///    pub fn cost(&self) -> Option<u32> {
///       match self {
///          Self::Mountain => None, // Moutains cannot be traversed
///          Self::Plains => Some(0),
///          Self::Forest => Some(1),
///          Self::Desert => Some(2)
///       }
///    }
/// }
///
/// let start = hex(0, 0);
/// let movement_budget = 5u32;
/// let mut biomes: HashMap<Hex, Biome> = HashMap::new();
/// // Set coordinate biomes
/// // biomes.insert(hex(1, 2), Biome::Mountain);
/// // ..
/// let reachable_tiles = field_of_movement(start, movement_budget, |h| biomes.get(&h).and_then(|b| b.cost()));
/// ```
pub fn field_of_movement(
    coord: Hex,
    budget: u32,
    cost: impl Fn(Hex) -> Option<u32>,
) -> HashSet<Hex> {
    let mut computed_costs = HashMap::new();
    computed_costs.insert(coord, 0);
    (1..=budget)
        .flat_map(|i| coord.ring(i))
        .filter_map(|coord| {
            let coord_cost = cost(coord)?;
            let neighbor_cost = coord
                .all_neighbors()
                .into_iter()
                .filter_map(|n| computed_costs.get(&n))
                .min()?;
            let computed_cost = coord_cost + 1 + neighbor_cost;
            if computed_cost <= budget {
                computed_costs.insert(coord, computed_cost);
                Some(coord)
            } else {
                None
            }
        })
        .collect()
}
