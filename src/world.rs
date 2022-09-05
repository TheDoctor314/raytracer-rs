//! Describes the world - a collection of all the objects in a scene
//! and the routines for intersecting that world with a ray and computing
//! the colours.

use crate::{
    hit_list::{HitList, HitState},
    lights::PointLight,
    ray::Ray,
    sphere::Sphere,
    Color,
};

/// A collection of objects and lights in a scene.
#[derive(Debug)]
pub struct World {
    pub(crate) objects: Vec<Sphere>,
    pub(crate) lights: Vec<PointLight>,
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

    /// Returns the shade for an intersection.
    /// Iterates over all the light sources, calling `lighting()` for each.
    ///
    /// The necessary data is provided by `HitState`.
    pub fn shade_hit(&self, state: HitState<'_>) -> Color {
        self.lights
            .iter()
            .map(|l| {
                state
                    .obj
                    .material()
                    .lighting(l, state.point, state.eyev, state.normal)
            })
            .fold(Color::BLACK, |acc, c| acc + c)
    }

    /// Intersects the world with the given ray and returns the colour
    /// at the resulting intersection.
    pub fn color_at(&self, r: &Ray) -> Color {
        let mut xs = self.intersect(r);

        if let Some(hit) = xs.hit() {
            let state = hit.prepare_computations(r);
            self.shade_hit(state)
        } else {
            Color::BLACK
        }
    }
}
