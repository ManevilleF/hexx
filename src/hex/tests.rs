use super::*;

#[test]
fn hex_addition() {
    assert_eq!(Hex::ZERO + Hex::ZERO, Hex::ZERO);
    assert_eq!(Hex::ZERO + Hex::ONE, Hex::ONE);
    assert_eq!(Hex::ONE + Hex::ONE, Hex::new(2, 2));
    assert_eq!(Hex::ONE + Hex::new(3, 4), Hex::new(4, 5));
}

#[test]
fn int_addition() {
    assert_eq!(Hex::ZERO + 1, Hex::ONE);
    assert_eq!(Hex::ONE + 1, Hex::new(2, 2));
}

#[test]
fn hex_sum() {
    // zero sum
    assert_eq!(Hex::ZERO.line_to(Hex::ZERO).sum::<Hex>(), Hex::ZERO);
    // correct sum
    assert_eq!(Hex::ZERO.line_to(Hex::X).sum::<Hex>(), Hex::X);
    assert_eq!(Hex::ZERO.line_to(Hex::Y).sum::<Hex>(), Hex::Y);
    assert_eq!(Hex::ZERO.line_to(Hex::ONE).sum::<Hex>(), Hex::new(1, 2));
    assert_eq!(
        Hex::ZERO.line_to(Hex::new(5, 0)).sum::<Hex>(),
        Hex::new(15, 0)
    );
}

#[test]
fn hex_product() {
    assert_eq!(
        Hex::X.line_to(Hex::new(5, 0)).product::<Hex>(),
        Hex::new((1..=5).product(), 0)
    );
}

#[test]
fn hex_length() {
    assert_eq!(Hex::ZERO.length(), 0);
    assert_eq!(Hex::ZERO.ulength(), 0);
    assert_eq!(Hex::ONE.length(), 2);
    assert_eq!(Hex::ONE.ulength(), 2);
    assert_eq!((Hex::ONE * 100).length(), 200);
    assert_eq!((Hex::ONE * 100).ulength(), 200);

    assert_eq!(Hex::new(i32::MAX, 0).length(), i32::MAX);
    assert_eq!(Hex::new(i32::MAX, 0).ulength(), u32::MAX / 2);
    assert_eq!(Hex::new(i32::MIN + 1, 0).length(), i32::MAX);
    assert_eq!(Hex::new(i32::MIN + 1, 0).ulength(), u32::MAX / 2);
}

#[test]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
fn hex_avg_center() {
    let points = [
        Hex::ONE,
        Hex::new(5, -12),
        Hex::new(15, 2),
        Hex::new(-5, 24),
        Hex::new(-1, 17),
    ];
    let mean = points.iter().sum::<Hex>() / points.len() as i32;
    let center = Hex::new(10, 12) / 2;

    assert_eq!(points.into_iter().average(), mean);
    assert_eq!(points.into_iter().center(), center);
    assert_ne!(center, mean);

    for r in 0..30 {
        assert_eq!(Hex::ZERO.range(r).average(), Hex::ZERO);
        assert_eq!(Hex::ZERO.range(r).center(), Hex::ZERO);
    }
}

#[test]
fn hex_subtraction() {
    assert_eq!(Hex::ZERO - Hex::ZERO, Hex::ZERO);
    assert_eq!(Hex::ONE - Hex::ZERO, Hex::ONE);
    assert_eq!(Hex::ONE - Hex::ONE, Hex::ZERO);
    assert_eq!(Hex::ONE - Hex::new(2, 2), Hex::new(-1, -1));
    assert_eq!(Hex::ONE - Hex::new(4, 5), Hex::new(-3, -4));
}

#[test]
fn int_subtraction() {
    assert_eq!(Hex::ONE - 1, Hex::ZERO);
    assert_eq!(Hex::ONE - 2, Hex::splat(-1));
    assert_eq!(Hex::ZERO - 10, Hex::splat(-10));
}

#[test]
fn hex_multiplication() {
    assert_eq!(Hex::ONE * Hex::ZERO, Hex::ZERO);
    assert_eq!(Hex::ONE * Hex::ONE, Hex::ONE);
    assert_eq!(Hex::ONE * Hex::new(2, 2), Hex::new(2, 2));
    assert_eq!(Hex::ONE * Hex::new(5, 6), Hex::new(5, 6));
    assert_eq!(Hex::new(2, 3) * Hex::new(5, 10), Hex::new(10, 30));
}

#[test]
fn int_multiplication() {
    assert_eq!(Hex::ONE * 5, Hex::splat(5));
}

#[test]
fn hex_division() {
    assert_eq!(Hex::ONE / Hex::ONE, Hex::ONE);
    assert_eq!(Hex::new(2, 2) / Hex::new(2, 2), Hex::ONE);
    assert_eq!(Hex::new(10, 30) / Hex::new(2, 6), Hex::new(5, 5));
    assert_eq!(Hex::new(11, 31) / Hex::new(2, 6), Hex::new(5, 5));
}

#[test]
#[allow(clippy::cast_precision_loss)]
fn int_division() {
    assert_eq!(Hex::new(2, 2) / 2, Hex::ONE);
    assert_eq!(Hex::new(10, 30) / 2, Hex::new(5, 15));
    assert_eq!(Hex::new(11, 31) / 2, Hex::new(5, 16));

    for x in 0..30 {
        for y in 0..30 {
            let p = Hex { x, y };
            for d in 1..30 {
                let expected_len = p.length() / d;
                let res_int = p / d;
                let res_float = p / d as f32;
                let res_lerp = Hex::ZERO.lerp(p, expected_len as f32 / p.length() as f32);

                assert_eq!(res_int.length(), expected_len);
                assert_eq!(res_int, res_float);
                assert_eq!(res_int, res_lerp);
            }
        }
    }
}

#[test]
fn hex_rem() {
    for x in 1..30 {
        for y in 1..30 {
            let p = Hex::new(x, y);
            for x2 in 1..30 {
                for y2 in 1..30 {
                    // Int
                    let rhs = x2;
                    let div = p / rhs;
                    let rem = p % rhs;
                    assert_eq!(div * rhs + rem, p);
                    // Hex
                    let rhs = Hex::new(x2, y2);
                    let div = p / rhs;
                    let rem = p % rhs;
                    assert_eq!(div * rhs + rem, p);
                }
            }
        }
    }
}

#[test]
fn neighbors() {
    assert_eq!(
        Hex::ZERO.all_neighbors(),
        [
            Hex::new(1, -1),
            Hex::new(0, -1),
            Hex::new(-1, 0),
            Hex::new(-1, 1),
            Hex::new(0, 1),
            Hex::new(1, 0),
        ]
    );
    assert_eq!(
        Hex::new(-2, 5).all_neighbors(),
        [
            Hex::new(-1, 4),
            Hex::new(-2, 4),
            Hex::new(-3, 5),
            Hex::new(-3, 6),
            Hex::new(-2, 6),
            Hex::new(-1, 5),
        ]
    );
}

#[test]
fn diagonals() {
    assert_eq!(
        Hex::ZERO.all_diagonals(),
        [
            Hex::new(2, -1),
            Hex::new(1, -2),
            Hex::new(-1, -1),
            Hex::new(-2, 1),
            Hex::new(-1, 2),
            Hex::new(1, 1),
        ]
    );
    assert_eq!(
        Hex::new(-2, 5).all_diagonals(),
        [
            Hex::new(0, 4),
            Hex::new(-1, 3),
            Hex::new(-3, 4),
            Hex::new(-4, 6),
            Hex::new(-3, 7),
            Hex::new(-1, 6),
        ]
    );
}

#[test]
fn distance_to() {
    assert_eq!(Hex::ZERO.distance_to(Hex::ZERO), 0);
    assert_eq!(Hex::ZERO.distance_to(Hex::ONE), 2);
    assert_eq!(Hex::ZERO.distance_to(Hex::new(2, 2)), 4);
    assert_eq!(Hex::ZERO.distance_to(Hex::new(-2, -2)), 4);
    assert_eq!(Hex::new(-2, -2).distance_to(Hex::new(-4, -4)), 4);
}

#[test]
fn rotation() {
    let neighbors = Hex::ZERO.all_neighbors();
    for elems in neighbors.windows(2) {
        let [next, prev] = [elems[0], elems[1]];
        let prev_dir = Hex::ZERO.way_to(prev).unwrap();
        let next_dir = Hex::ZERO.way_to(next).unwrap();
        assert_eq!(prev.right(), next);
        assert_eq!(next.left(), prev);
        assert_eq!(prev_dir.right(), next_dir);
        assert_eq!(next_dir.left(), prev_dir);
    }
}

#[test]
fn rotate_right() {
    let point = Hex::new(5, 0);
    let new = point.right();
    assert_eq!(new, Hex::new(0, 5));
    assert_eq!(point.rotate_right(1), new);
    let new = new.right();
    assert_eq!(new, Hex::new(-5, 5));
    assert_eq!(point.rotate_right(2), new);
    let new = new.right();
    assert_eq!(new, Hex::new(-5, 0));
    assert_eq!(point.rotate_right(3), new);
    let new = new.right();
    assert_eq!(new, Hex::new(0, -5));
    assert_eq!(point.rotate_right(4), new);
    let new = new.right();
    assert_eq!(new, Hex::new(5, -5));
    assert_eq!(point.rotate_right(5), new);
    let new = new.right();
    assert_eq!(new, point);
    assert_eq!(point.rotate_right(6), new);
    assert_eq!(point.rotate_right(7), point.rotate_right(1));
    assert_eq!(point.rotate_right(10), point.rotate_right(4));
}

#[test]
fn rotate_left() {
    let point = Hex::new(5, 0);
    let new = point.left();
    assert_eq!(new, Hex::new(5, -5));
    assert_eq!(point.rotate_left(1), new);
    let new = new.left();
    assert_eq!(new, Hex::new(0, -5));
    assert_eq!(point.rotate_left(2), new);
    let new = new.left();
    assert_eq!(new, Hex::new(-5, 0));
    assert_eq!(point.rotate_left(3), new);
    let new = new.left();
    assert_eq!(new, Hex::new(-5, 5));
    assert_eq!(point.rotate_left(4), new);
    let new = new.left();
    assert_eq!(new, Hex::new(0, 5));
    assert_eq!(point.rotate_left(5), new);
    let new = new.left();
    assert_eq!(new, point);
    assert_eq!(point.rotate_left(6), new);
    assert_eq!(point.rotate_left(7), point.rotate_left(1));
    assert_eq!(point.rotate_left(10), point.rotate_left(4));
}

#[test]
fn lerp() {
    let a = Hex::new(0, 0);
    let b = Hex::new(5, 0);

    assert_eq!(a.lerp(b, 0.0), a);
    assert_eq!(a.lerp(b, 1.0), b);
    assert_eq!(a.lerp(b, 2.0), b * 2);
    assert_eq!(a.lerp(b, -1.0), -b);
    assert_eq!(a.lerp(b, -2.0), -b * 2);

    let line = [
        a,
        Hex::new(1, 0),
        Hex::new(2, 0),
        Hex::new(3, 0),
        Hex::new(4, 0),
        b,
    ];
    assert_eq!(a.lerp(b, 0.1), line[0]);
    assert_eq!(a.lerp(b, 0.2), line[1]);
    assert_eq!(a.lerp(b, 0.3), line[1]);
    assert_eq!(a.lerp(b, 0.4), line[2]);
    assert_eq!(a.lerp(b, 0.5), line[2]);
    assert_eq!(a.lerp(b, 0.6), line[3]);
    assert_eq!(a.lerp(b, 0.7), line[3]);
    assert_eq!(a.lerp(b, 0.8), line[4]);
    assert_eq!(a.lerp(b, 0.9), line[4]);
    assert_eq!(a.lerp(b, 0.95), line[5]);
    assert_eq!(a.lerp(b, 1.0), line[5]);
}

#[test]
fn line_to() {
    let a = Hex::new(0, 0);
    let b = Hex::new(5, 0);
    let line = a.line_to(b);
    let len = line.len();
    let line: Vec<_> = line.collect();
    assert_eq!(line.len(), len);
    assert_eq!(
        line,
        vec![
            a,
            Hex::new(1, 0),
            Hex::new(2, 0),
            Hex::new(3, 0),
            Hex::new(4, 0),
            b
        ]
    );
    let b = Hex::new(5, 5);
    let line = a.line_to(b);
    let len = line.len();
    let line: Vec<_> = line.collect();
    assert_eq!(line.len(), len);
    assert_eq!(
        line,
        vec![
            a,
            Hex::new(0, 1),
            Hex::new(1, 1),
            Hex::new(1, 2),
            Hex::new(2, 2),
            Hex::new(2, 3),
            Hex::new(3, 3),
            Hex::new(3, 4),
            Hex::new(4, 4),
            Hex::new(4, 5),
            b
        ]
    );
}

#[test]
fn empty_line_to() {
    let start = Hex::new(3, -7);
    let line = start.line_to(start);
    assert_eq!(line.len(), 1);
    assert_eq!(line.collect::<Vec<_>>(), vec![start]);
}

#[test]
fn range_count() {
    assert_eq!(Hex::range_count(0), 1);
    assert_eq!(Hex::range_count(1), 7);
    assert_eq!(Hex::range_count(10), 331);
    assert_eq!(Hex::range_count(15), 721);
}

#[test]
fn range() {
    let point = Hex::new(13, -54);
    let mut range = point.range(16);
    assert_eq!(range.len(), Hex::range_count(16));
    assert_eq!(range.size_hint(), (range.len(), Some(range.len())));
    println!("{:#?}", range.size_hint());
    range.next();
    println!("{:#?}", range.size_hint());
    assert_eq!(range.len(), Hex::range_count(16) - 1);
    assert_eq!(range.size_hint(), (range.len(), Some(range.len())));
    range.next();
    assert_eq!(range.len(), Hex::range_count(16) - 2);
    assert_eq!(range.size_hint(), (range.len(), Some(range.len())));
}

#[test]
fn ring() {
    let point = Hex::ZERO;
    assert_eq!(point.ring(0).collect::<Vec<_>>(), vec![point]);
    let expected = point.all_neighbors().to_vec();
    assert_eq!(point.ring(1).collect::<Vec<_>>(), expected);

    let radius = 5;
    let mut range: Vec<_> = point.range(radius).collect();
    let removed: Vec<_> = point.range(radius - 1).collect();
    range.retain(|h| !removed.contains(h));
    let ring = point.ring(5);
    assert_eq!(ring.len(), range.len());
    for h in ring {
        assert!(range.contains(&h));
    }
}

#[test]
#[allow(clippy::cast_possible_truncation)]
fn cached_rings() {
    let point = Hex::ZERO;
    let cache = point.cached_rings::<10>();
    for (r, ring) in cache.into_iter().enumerate() {
        let expected: Vec<_> = point.ring(r as u32).collect();
        assert_eq!(ring, expected);
    }
}

#[test]
fn ring_offset() {
    let zero = Hex::ZERO;
    let target = Hex::new(14, 7);

    let expected: Vec<_> = zero.ring(10).map(|h| h + target).collect();
    assert_eq!(target.ring(10).collect::<Vec<_>>(), expected);
}

#[test]
fn custom_ring() {
    let point = Hex::ZERO;
    assert_eq!(
        point
            .custom_ring(0, Direction::TopLeft, true)
            .collect::<Vec<_>>(),
        vec![point]
    );

    // clockwise
    let mut expected: Vec<_> = point.ring(5).collect();
    expected.reverse();
    expected.rotate_right(1);
    assert_eq!(
        point
            .custom_ring(5, Direction::TopRight, true)
            .collect::<Vec<_>>(),
        expected
    );
    // offsetted
    let expected: Vec<_> = point.ring(5).collect();
    let ring = point.custom_ring(5, Direction::BottomLeft, false);
    assert_eq!(expected.len(), ring.len());
    for h in ring {
        assert!(expected.contains(&h));
    }
}

#[test]
fn ring_edge() {
    let point = Hex::new(-189, 35);
    let edge = point.ring_edge(48, DiagonalDirection::TopRight);
    assert_eq!(edge.len(), edge.count());
    // empty
    let edge = point.ring_edge(0, DiagonalDirection::TopRight);
    let len = edge.len();
    let edge: Vec<_> = edge.collect();
    assert_eq!(edge.len(), len);
    assert_eq!(edge, vec![point]);
    // len 1
    let edge = point.ring_edge(1, DiagonalDirection::TopRight);
    let len = edge.len();
    let edge: Vec<_> = edge.collect();
    assert_eq!(edge.len(), len);
    assert_eq!(edge.len(), 2);
}

#[test]
#[allow(clippy::cast_possible_truncation)]
fn wedge() {
    let point = Hex::new(98, -123);
    for dir in DiagonalDirection::ALL_DIRECTIONS {
        for range in 0..=30 {
            let wedge: Vec<_> = point.wedge(0..=range, dir).collect();
            assert_eq!(wedge.len() as u32, range * (range + 3) / 2 + 1);
            assert_eq!(wedge.len() as u32, Hex::wedge_count(range));
            let full_wedge = point.full_wedge(range, dir);
            assert_eq!(full_wedge.len(), wedge.len());
        }
    }
}

#[test]
fn spiral_range() {
    let expected: Vec<_> = Hex::ZERO.range(10).collect();
    let spiral: Vec<_> = Hex::ZERO.spiral_range(0..=10).collect();
    assert_eq!(spiral.len(), expected.len());
    for hex in &expected {
        assert!(spiral.contains(hex));
    }
}
