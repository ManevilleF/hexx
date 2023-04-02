use crate::Hex;
use std::collections::{BinaryHeap, HashMap};

struct Node {
    coord: Hex,
    cost: u32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        rhs.cost.partial_cmp(&self.cost)
    }
}

impl Ord for Node {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        rhs.cost.cmp(&self.cost)
    }
}

fn reconstruct_path(came_from: &HashMap<Hex, Hex>, end: Hex) -> Vec<Hex> {
    let mut path: Vec<_> =
        std::iter::successors(Some(end), move |&current| came_from.get(&current).copied())
            .collect();
    path.reverse();
    path
}

/// Performs A star pathfinding between `start` and `end`.
/// The `cost` parameter should give the cost of each coordinate (`Some`) or indicate the
/// coordinate is not included in the pathfinding (`None`)
///
/// # Examples
///
/// - Compute a A star with no boundaries and some forbidden tiles
///
/// ```rust
/// # use hexx::*;
/// use hexx::algorithms::a_star;
///
/// let start = hex(0, 0);
/// let end = hex(10, 0);
/// let forbidden_coords: HashSet<Hex> = vec![hex(2, 0), hex(3,0), hex(1, 1)].into();
/// let path = a_star(start, end, |h| (!forbidden_coords.contains(&h)).then_some(0));
/// ```
pub fn a_star(start: Hex, end: Hex, cost: impl Fn(Hex) -> Option<u32>) -> Option<Vec<Hex>> {
    let heuristic = |h: Hex| h.unsigned_distance_to(start);

    let start_node = Node {
        coord: start,
        cost: heuristic(start),
    };
    let mut open = BinaryHeap::new();
    let mut costs = HashMap::new();
    costs.insert(start, start_node.cost);
    let mut came_from = HashMap::new();
    open.push(start_node);

    while let Some(node) = open.pop() {
        if node.coord == end {
            return Some(reconstruct_path(&came_from, end));
        }
        for neighbor in node.coord.all_neighbors() {
            let Some(cost) = cost(neighbor) else { continue };
            let neighbor_cost = costs[&node.coord] + cost + heuristic(neighbor);
            if !costs.contains_key(&neighbor) || costs[&neighbor] > neighbor_cost {
                came_from.insert(neighbor, node.coord);
                costs.insert(neighbor, neighbor_cost);
                open.push(Node {
                    coord: neighbor,
                    cost: neighbor_cost,
                });
            }
        }
    }
    None
}
