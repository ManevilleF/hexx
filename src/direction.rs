/// All 6 possible directions in hexagonal space
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "ser_de", derive(serde::Serialize, serde::Deserialize))]
pub enum Direction {
    /// (1, 0)
    BottomRight = 0,
    /// (1, -1)
    TopRight = 1,
    /// (0, -1)
    Top = 2,
    /// (-1, 0)
    TopLeft = 3,
    /// (-1, 1)
    BottomLeft = 4,
    /// (0, 1)
    Bottom = 5,
}

impl Direction {
    /// Iterates through all enum variant in order
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::BottomRight,
            Self::TopRight,
            Self::Top,
            Self::TopLeft,
            Self::BottomLeft,
            Self::Bottom,
        ]
        .into_iter()
    }
}
