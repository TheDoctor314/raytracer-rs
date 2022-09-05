//! Describes the world - a collection of all the objects in a scene
//! and the routines for intersecting that world with a ray and computing
//! the colours.

use crate::{hit_list::HitList, lights::PointLight, ray::Ray, sphere::Sphere};

/// A collection of objects and lights in a scene.
#[derive(Debug)]
pub struct World {
    objects: Vec<Sphere>,
    lights: Vec<PointLight>,
}

impl World {
    /// Constructs a new `World`.
    pub fn new(objects: Vec<Sphere>, lights: Vec<PointLight>) -> Self {
        Self { objects, lights }
    }

    /// Intersects the ray with the sphere.
    /// Returns a `HitList` with the intersections in sorted order.
    pub fn intersect(&self, r: &Ray) -> HitList<'_> {
        let hits = self
            .objects
            .iter()
            .flat_map(|s| s.intersect(r).into_inner())
            .collect();

        let mut hits = HitList::new(hits);
        hits.sort();

        hits
    }
}
