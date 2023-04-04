use crate::{DiagonalDirection, Direction, Hex};
use std::collections::HashSet;

pub fn range_fov(coord: Hex, range: u32, visible: impl Fn(Hex) -> bool) -> HashSet<Hex> {
    coord
        .ring(range)
        .flat_map(|target| coord.line_to(target).take_while(|h| visible(*h)))
        .collect()
}

pub fn dual_fov(
    coord: Hex,
    range: u32,
    visible: impl Fn(Hex) -> bool,
    direction: Direction,
) -> HashSet<Hex> {
    let [a, b] = [direction.diagonal_left(), direction.diagonal_right()];
    coord
        .ring(range)
        .filter(|h| {
            let way = coord.diagonal_way_to(*h);
            way == a || way == b
        })
        .flat_map(|target| coord.line_to(target).take_while(|h| visible(*h)))
        .collect()
}

pub fn directional_fov(
    coord: Hex,
    range: u32,
    visible: impl Fn(Hex) -> bool,
    direction: Direction,
) -> HashSet<Hex> {
    coord
        .ring(range)
        .filter(|h| coord.way_to(*h) == direction)
        .flat_map(|target| coord.line_to(target).take_while(|h| visible(*h)))
        .collect()
}

pub fn diagonal_fov(
    coord: Hex,
    range: u32,
    visible: impl Fn(Hex) -> bool,
    direction: DiagonalDirection,
) -> HashSet<Hex> {
    coord
        .ring(range)
        .filter(|h| coord.diagonal_way_to(*h) == direction)
        .flat_map(|target| coord.line_to(target).take_while(|h| visible(*h)))
        .collect()
}
