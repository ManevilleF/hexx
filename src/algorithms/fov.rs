use crate::Hex;
use std::collections::HashSet;

pub fn field_of_view(coord: Hex, range: u32, visible: impl Fn(Hex) -> bool) -> HashSet<Hex> {
    coord
        .ring(range)
        .flat_map(|target| coord.line_to(target).take_while(|h| visible(*h)))
        .collect()
}
