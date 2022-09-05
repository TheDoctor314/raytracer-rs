//! This module is responsible for storing the data related to the intersection
//! of objects with the rays.

use std::ops;

use crate::{
    ray::Ray,
    sphere::Sphere,
    vec3::{Point3, Vec3},
};

/// Stores data related to intersections.
#[derive(Debug, PartialEq)]
pub struct HitRec<'a> {
    /// How far along the ray the intersection occurred.
    pub t: f32,
    /// The object of intersection.
    pub obj: &'a Sphere,
}

impl<'a> HitRec<'a> {
    /// Constructs a `HitState` to make it easier to reuse computations
    /// for an intersection.
    pub fn prepare_computations(&self, r: &Ray) -> HitState<'_> {
        let t = self.t;
        let obj = self.obj;
        let point = r.pos(t);
        let eyev = -r.dir;
        let mut normal = obj.normal_at(point);

        let inside = if normal.dot(eyev) < 0.0 {
            normal = -normal;
            true
        } else {
            false
        };

        HitState {
            t,
            obj,
            point,
            eyev,
            normal,
            inside,
        }
    }
}

/// A collection of `HitRec`s.
#[derive(Debug, Default)]
pub struct HitList<'a> {
    inner: Vec<HitRec<'a>>,
}

impl<'a> HitList<'a> {
    /// Creates a new `HitList`.
    pub fn new(recs: Vec<HitRec<'a>>) -> Self {
        Self { inner: recs }
    }

    /// Gets the inner vector from the `HitList`.
    pub fn into_inner(self) -> Vec<HitRec<'a>> {
        self.inner
    }

    /// Sorts the list of `HitRec`s.
    pub fn sort(&mut self) {
        self.inner
            .sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    }

    /// Returns the record with the smallest non-negative `t`.
    pub fn hit(&mut self) -> Option<&HitRec<'_>> {
        self.sort();
        self.inner.iter().find(|h| h.t >= 0.0)
    }

    /// Returns the number of `HitRec`s stored.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the list is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<'a> ops::Index<usize> for HitList<'a> {
    type Output = HitRec<'a>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<'a> ops::IndexMut<usize> for HitList<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

/// Stores some data for an intersection which can be reused in
/// different calculations.
#[derive(Debug)]
pub struct HitState<'a> {
    pub t: f32,
    pub obj: &'a Sphere,
    pub point: Point3,
    pub eyev: Vec3,
    pub normal: Vec3,
    pub inside: bool,
}
