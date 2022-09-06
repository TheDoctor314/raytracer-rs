//! Defines all the unit tests for this library.
//! They are in a separate module so that the whole library
//! is not recompiled for running tests.

mod camera;
mod material;
mod matrix;
mod ray;
mod sphere;
mod vec3;
mod world;

use approx::assert_relative_eq;

use crate::Color;

#[test]
fn color() {
    let a: Color = [0.9, 0.6, 0.75].into();
    let b: Color = [0.7, 0.1, 0.25].into();

    assert_relative_eq!(a + b, [1.6, 0.7, 1.0].into());
    assert_relative_eq!(a - b, [0.2, 0.5, 0.5].into());

    let c: Color = [0.2, 0.3, 0.4].into();
    assert_relative_eq!(c * 2.0, [0.4, 0.6, 0.8].into());

    let a: Color = [1.0, 0.2, 0.4].into();
    let b: Color = [0.9, 1.0, 0.1].into();

    assert_relative_eq!(a.blend(b), [0.9, 0.2, 0.04].into());
}

mod hit_list {
    use crate::{hit_list::HitRec, ray::Ray, sphere::Sphere};

    #[test]
    fn precompute() {
        let r = Ray::new((0., 0., -5.), (0., 0., 1.));
        let s = Sphere::default();

        let hit = HitRec { t: 4.0, obj: &s };

        let hit_state = hit.prepare_computations(&r);

        assert_eq!(hit_state.t, hit.t);
        assert!(std::ptr::eq(hit_state.obj, hit.obj));
        assert_eq!(hit_state.point, (0., 0., -1.).into());
        assert_eq!(hit_state.eyev, (0., 0., -1.).into());
        assert_eq!(hit_state.normal, (0., 0., -1.).into());
    }

    #[test]
    fn hit_from_outside_the_sphere() {
        let r = Ray::new((0., 0., -5.), (0., 0., 1.));
        let s = Sphere::default();

        let hit = HitRec { t: 4.0, obj: &s };

        let hit_state = hit.prepare_computations(&r);

        assert!(!hit_state.inside);
    }

    #[test]
    fn hit_from_inside_the_sphere() {
        let r = Ray::new((0., 0., 0.), (0., 0., 1.));
        let s = Sphere::default();

        let hit = HitRec { t: 1.0, obj: &s };

        let hit_state = hit.prepare_computations(&r);

        assert!(hit_state.inside);
        assert_eq!(hit_state.point, (0., 0., 1.).into());
        assert_eq!(hit_state.eyev, (0., 0., -1.).into());

        // would have (0., 0., 1.) but is the opposite
        assert_eq!(hit_state.normal, (0., 0., -1.).into());
    }
}
