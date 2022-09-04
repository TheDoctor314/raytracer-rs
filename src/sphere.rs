//! Implementation of the sphere object.

use crate::{
    hit_list::{HitList, HitRec},
    matrix::Mat4,
    ray::Ray,
    vec3::{Point3, Vec3},
};

/// Representation of a sphere and its associated transform.
/// The default sphere is always a unit sphere at the origin.
/// We use the transform to create different configurations of the sphere.
#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    transform: Mat4,
    transform_inv: Mat4,
}

impl Sphere {
    /// Constructs a new `Sphere`.
    pub fn new() -> Self {
        Self {
            transform: Mat4::identity(),
            transform_inv: Mat4::identity(),
        }
    }

    /// Set the transform of a `Sphere`.
    pub fn with_transform(mut self, transform: Mat4) -> Self {
        self.transform_inv = transform.inverse().unwrap_or_else(Mat4::identity);
        self.transform = transform;
        self
    }

    /// Intersect the ray with the sphere.
    /// Returns a `HitList` which stores the point and object of intersections.
    pub fn intersect(&self, r: &Ray) -> HitList<'_> {
        let r = r.transform(&self.transform_inv);

        let sphere_to_ray = r.orig - Point3::new(0.0, 0.0, 0.0);

        let a = r.dir.dot(r.dir);
        let b = 2.0 * r.dir.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let d = b * b - 4.0 * a * c;

        if d.is_sign_negative() {
            HitList::default()
        } else {
            let t1 = (-b - d.sqrt()) / (2.0 * a);
            let t2 = (-b + d.sqrt()) / (2.0 * a);

            let t1 = HitRec { t: t1, obj: self };

            let t2 = HitRec { t: t2, obj: self };

            HitList::new(vec![t1, t2])
        }
    }

    /// Returns the normal at a point on the sphere.
    pub fn normal_at(&self, point: Point3) -> Vec3 {
        let object_point = &self.transform_inv * point;
        let object_normal = object_point - Point3::default();

        let mut world_normal = &self.transform_inv.transpose() * object_normal;

        // this is a bit of a hack because we should technically be multiplying by
        // the transpose inverse of the submatrix of the transform to disregard
        // any translation. But, we avoid all that by simply setting the last field of the vector
        // to zero.
        world_normal[3] = 0.0;

        world_normal.normalize()
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}
