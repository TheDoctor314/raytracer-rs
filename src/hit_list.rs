//! This module is responsible for storing the data related to the intersection
//! of objects with the rays.

use std::ops::Deref;

use crate::sphere::Sphere;

/// Stores data related to intersections.
#[derive(Debug, PartialEq)]
pub struct HitRec<'a> {
    /// How far along the ray the intersection occurred.
    pub t: f32,
    /// The object of intersection.
    pub obj: &'a Sphere,
}

/// A collection of `HitRec`s.
#[derive(Debug, Default)]
pub struct HitList<'a> {
    inner: Vec<HitRec<'a>>,
}

impl<'a> Deref for HitList<'a> {
    type Target = Vec<HitRec<'a>>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> HitList<'a> {
    /// Creates a new `HitList`.
    pub fn new(recs: Vec<HitRec<'a>>) -> Self {
        Self { inner: recs }
    }

    /// Returns the record with the smallest non-negative `t`.
    pub fn hit(&mut self) -> Option<&HitRec<'_>> {
        self.inner
            .sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        self.inner.iter().find(|h| h.t >= 0.0)
    }
}
