/// Hexagonal neighbor/edge directions
mod edge_direction;
/// Trait implementations
mod impls;
/// Test module
#[cfg(test)]
mod tests;
/// Hexagonal vertex/diagonal directions
mod vertex_direction;
/// Direction way module
pub(crate) mod way;

pub use edge_direction::EdgeDirection;
pub use vertex_direction::VertexDirection;
pub use way::DirectionWay;

/// Angle constants used for directions
pub mod angles {
    /// Angle in radian between *flat* and *pointy* top orientations.
    /// Equivalent to 30 degrees
    pub const DIRECTION_ANGLE_OFFSET_RAD: f32 = std::f32::consts::FRAC_PI_6;
    /// Angle in radian between *flat* and *pointy* top orientations.
    /// Equivalent to π / 6 in radians
    pub const DIRECTION_ANGLE_OFFSET_DEGREES: f32 = 30.0;
    /// Angle in radian between two adjacent directions counter clockwise.
    /// Equivalent to 60 degrees
    pub const DIRECTION_ANGLE_RAD: f32 = std::f32::consts::FRAC_PI_3;
    /// Angle in degrees between two adjacent directions counter clockwise.
    /// Equivalent to π / 3 in radians
    pub const DIRECTION_ANGLE_DEGREES: f32 = 60.0;
}
