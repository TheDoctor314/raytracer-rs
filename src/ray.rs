//! Implementation of a ray and its operations.

use crate::{
    matrix::Mat4,
    vec3::{Point3, Vec3},
};

/// Representation of a ray.
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

    /// Transform the ray using the given transform.
    pub fn transform(&self, transform: &Mat4) -> Self {
        Self {
            orig: transform * self.orig,
            dir: transform * self.dir,
        }
    }

    /// Gets the origin of the ray.
    pub fn orig(&self) -> &Point3 {
        &self.orig
    }

    /// Gets the direction of the ray.
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
}
