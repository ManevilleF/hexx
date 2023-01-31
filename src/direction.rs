/// All 6 possible directions in hexagonal space.
///
/// ```txt
///            x Axis
///            ___
///           /   \
///       +--+  2  +--+
///      / 3  \___/  1 \
///      \    /   \    /
///       +--+     +--+
///      /    \___/    \
///      \ 4  /   \  0 /
///       +--+  5  +--+   y Axis
///           \___/
/// ```
///
/// See [`Hex::NEIGHBORS_COORDS`]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize, serde::Deserialize))]
pub enum Direction {
    /// (1, 0)
    /// ```txt
    ///            x Axis
    ///            ___
    ///           /   \
    ///       +--+     +--+
    ///      /    \___/    \
    ///      \    /   \    /
    ///       +--+     +--+
    ///      /    \___/    \
    ///      \    /   \  X /
    ///       +--+     +--+   y Axis
    ///           \___/
    /// ```
    BottomRight = 0,
    /// (1, -1)
    /// ```txt
    ///            x Axis
    ///            ___
    ///           /   \
    ///       +--+     +--+
    ///      /    \___/  X \
    ///      \    /   \    /
    ///       +--+     +--+
    ///      /    \___/    \
    ///      \    /   \    /
    ///       +--+     +--+   y Axis
    ///           \___/
    /// ```
    TopRight = 1,
    /// (0, -1)
    /// ```txt
    ///            x Axis
    ///            ___
    ///           /   \
    ///       +--+  X  +--+
    ///      /    \___/    \
    ///      \    /   \    /
    ///       +--+     +--+
    ///      /    \___/    \
    ///      \    /   \    /
    ///       +--+     +--+   y Axis
    ///           \___/
    /// ```
    Top = 2,
    /// (-1, 0)
    /// ```txt
    ///            x Axis
    ///            ___
    ///           /   \
    ///       +--+     +--+
    ///      / X  \___/    \
    ///      \    /   \    /
    ///       +--+     +--+
    ///      /    \___/    \
    ///      \    /   \    /
    ///       +--+     +--+   y Axis
    ///           \___/
    /// ```
    TopLeft = 3,
    /// (-1, 1)
    /// ```txt
    ///            x Axis
    ///            ___
    ///           /   \
    ///       +--+     +--+
    ///      /    \___/    \
    ///      \    /   \    /
    ///       +--+     +--+
    ///      /    \___/    \
    ///      \ X  /   \    /
    ///       +--+     +--+   y Axis
    ///           \___/
    /// ```
    BottomLeft = 4,
    /// (0, 1)
    /// ```txt
    ///            x Axis
    ///            ___
    ///           /   \
    ///       +--+     +--+
    ///      /    \___/    \
    ///      \    /   \    /
    ///       +--+     +--+
    ///      /    \___/    \
    ///      \    /   \    /
    ///       +--+  X  +--+   y Axis
    ///           \___/
    /// ```
    Bottom = 5,
}

impl Direction {
    /// All 6 hexagonal directions matching [`Hex::NEIGHBORS_COORDS`]
    /// ```txt
    ///            x Axis
    ///            ___
    ///           /   \
    ///       +--+  2  +--+
    ///      / 3  \___/  1 \
    ///      \    /   \    /
    ///       +--+     +--+
    ///      /    \___/    \
    ///      \ 4  /   \  0 /
    ///       +--+  5  +--+   y Axis
    ///           \___/
    /// ```
    pub const ALL_DIRECTIONS: [Self; 6] = [
        Self::BottomRight,
        Self::TopRight,
        Self::Top,
        Self::TopLeft,
        Self::BottomLeft,
        Self::Bottom,
    ];

    /// Iterates through all enum variant in order
    pub fn iter() -> impl Iterator<Item = Self> {
        Self::ALL_DIRECTIONS.into_iter()
    }
}
