//! This modules represents the different kinds of lights in a scene.

use crate::{vec3::Point3, Color};

/// Represents a point light - a light source with no size existing at
/// a single point in space.
#[derive(Debug)]
pub struct PointLight {
    /// Position of the light.
    pub(crate) pos: Point3,
    /// Describes the brightness and colour of the light.
    pub(crate) intensity: Color,
}

impl PointLight {
    /// Constructs a new `PointLight`.
    pub fn new(pos: impl Into<Point3>, intensity: impl Into<Color>) -> Self {
        let pos = pos.into();
        let intensity = intensity.into();

        Self { pos, intensity }
    }
}
