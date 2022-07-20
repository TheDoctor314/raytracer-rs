//! Implementation of a ray and its operations.

use crate::vec3::{Point3, Vec3};

/// Representation of a ray of light.
#[derive(Debug, Clone)]
pub struct Ray {
    /// The point where the ray starts from.
    pub(crate) orig: Point3,
    /// The direction of the ray.
    pub(crate) dir: Vec3,
}

impl Ray {
    /// Constructs a new `Ray` from the given `Point3` and `Vec3`.
    pub fn new(orig: impl Into<Point3>, dir: impl Into<Vec3>) -> Self {
        let orig = orig.into();
        let dir = dir.into();

        Self { orig, dir }
    }

    /// Computes the point at a given distance `t` along the ray.
    pub fn pos(&self, t: f32) -> Point3 {
        self.orig + self.dir * t
    }
}
