use crate::{
    hit_list::{HitList, HitRec},
    ray::Ray,
    vec3::Point3,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Sphere {}

impl Sphere {
    pub fn new() -> Self {
        Sphere {}
    }

    pub fn intersect(&self, r: &Ray) -> HitList {
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
}
