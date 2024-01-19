#![allow(clippy::many_single_char_names)]
use super::Hex;
use wide::{i32x4, i32x8};

macro_rules! simd_op {
    ($name:ident, $num:expr, $vec_type:ident, $op:ident, $op_name:expr) => {
        impl Hex {
            #[doc = concat!("Performs a ", stringify!($op_name), " over all `vals` coordinates using SIMD instructions if possible. Operates on ", stringify!($num), " elements.")]
            #[must_use]
            pub fn $name(vals: [Self; $num]) -> Self {
                let x_values = vals.map(|h| h.x);
                let y_values = vals.map(|h| h.y);
                let x = $vec_type::new(x_values).$op();
                let y = $vec_type::new(y_values).$op();
                Self::new(x, y)
            }
        }
    };
}

// Usage for sum
simd_op!(simd_sum4, 4, i32x4, reduce_add, "sum");
simd_op!(simd_sum8, 8, i32x8, reduce_add, "sum");

// Usage for min
simd_op!(simd_min4, 4, i32x4, reduce_min, "min");
simd_op!(simd_min8, 8, i32x8, reduce_min, "min");

// Usage for max
simd_op!(simd_max4, 4, i32x4, reduce_max, "max");
simd_op!(simd_max8, 8, i32x8, reduce_max, "max");
