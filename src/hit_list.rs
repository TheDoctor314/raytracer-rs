use std::ops::Deref;

use crate::sphere::Sphere;

#[derive(Debug, PartialEq)]
pub struct HitRec<'a> {
    pub t: f32,
    pub obj: &'a Sphere,
}

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
    pub fn new(recs: Vec<HitRec<'a>>) -> Self {
        Self { inner: recs }
    }

    pub fn hit(&mut self) -> Option<&HitRec> {
        self.inner
            .sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        self.inner.iter().find(|h| h.t >= 0.0)
    }
}
