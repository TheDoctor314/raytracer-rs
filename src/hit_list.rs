//! This module is responsible for storing the data related to the intersection
//! of objects with the rays.

use std::ops;

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
