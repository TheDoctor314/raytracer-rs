use std::f32::consts;

use approx::assert_relative_eq;

use crate::{
    hit_list::{HitList, HitRec},
    matrix::Mat4,
    ray::Ray,
    sphere::Sphere,
};

#[test]
fn ray_intersect() {
    let r = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
    let s = Sphere::default();

    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);

    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 6.0);

    assert!(std::ptr::eq(xs[0].obj, &s));
}

#[test]
fn ray_intersect_tangent() {
    let r = Ray::new((0.0, 1.0, -5.0), (0.0, 0.0, 1.0));
    let s = Sphere::default();

    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);

    assert_eq!(xs[0].t, 5.0);
    assert_eq!(xs[1].t, 5.0);
}

#[test]
fn ray_miss() {
    let r = Ray::new((0.0, 2.0, -5.0), (0.0, 0.0, 1.0));
    let s = Sphere::default();

    let xs = s.intersect(&r);
    assert!(xs.is_empty());
}

#[test]
fn ray_in_sphere() {
    let r = Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
    let s = Sphere::default();

    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);

    assert_eq!(xs[0].t, -1.0);
    assert_eq!(xs[1].t, 1.0);
}

#[test]
fn ray_behind_sphere() {
    let r = Ray::new((0.0, 0.0, 5.0), (0.0, 0.0, 1.0));
    let s = Sphere::default();

    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);

    assert_eq!(xs[0].t, -6.0);
    assert_eq!(xs[1].t, -4.0);
}

#[test]
fn hit_with_positive_t() {
    let s = Sphere::default();
    let mut xs = HitList::new(vec![HitRec { t: 1.0, obj: &s }, HitRec { t: 2.0, obj: &s }]);

    assert_eq!(xs.hit(), Some(&HitRec { t: 1.0, obj: &s }));
}

#[test]
fn hit_with_negative_t() {
    {
        let s = Sphere::default();
        let mut xs = HitList::new(vec![
            HitRec { t: -1.0, obj: &s },
            HitRec { t: 1.0, obj: &s },
        ]);

        assert_eq!(xs.hit(), Some(&HitRec { t: 1.0, obj: &s }));
    }

    {
        let s = Sphere::default();
        let mut xs = HitList::new(vec![
            HitRec { t: -2.0, obj: &s },
            HitRec { t: -1.0, obj: &s },
        ]);

        assert_eq!(xs.hit(), None);
    }
}

#[test]
fn hit_with_t_in_random_order() {
    let s = Sphere::default();
    let mut xs = HitList::new(vec![
        HitRec { t: 5.0, obj: &s },
        HitRec { t: 7.0, obj: &s },
        HitRec { t: -3.0, obj: &s },
        HitRec { t: 2.0, obj: &s },
    ]);

    assert_eq!(xs.hit(), Some(&HitRec { t: 2.0, obj: &s }));
}

#[test]
fn ray_intersect_with_transformed_sphere() {
    let r = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
    let s = Sphere::default().with_transform(Mat4::new_scaling((2., 2., 2.).into()));

    let xs = s.intersect(&r);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 3.0);
    assert_eq!(xs[1].t, 7.0);

    let s = Sphere::default().with_transform(Mat4::new_translation((5., 0., 0.).into()));

    let xs = s.intersect(&r);

    assert!(xs.is_empty());
}

#[test]
fn normal() {
    let s = Sphere::default();

    let n = s.normal_at((1., 0., 0.).into());
    assert_relative_eq!(n, (1., 0., 0.).into());

    let n = s.normal_at((0., 1., 0.).into());
    assert_relative_eq!(n, (0., 1., 0.).into());

    let n = s.normal_at((0., 0., 1.).into());
    assert_relative_eq!(n, (0., 0., 1.).into());

    let a = 3.0f32.sqrt().recip();
    let n = s.normal_at((a, a, a).into());
    assert_relative_eq!(n, (a, a, a).into());

    // normals should always be normalized (lol).
    assert_relative_eq!(n, n.normalize());
}

#[test]
fn normal_at_transformed_sphere() {
    let s = Sphere::default().with_transform(Mat4::new_translation((0., 1., 0.).into()));

    let n = s.normal_at((0., consts::FRAC_1_SQRT_2 + 1.0, -consts::FRAC_1_SQRT_2).into());
    assert_relative_eq!(
        n,
        (0.0, consts::FRAC_1_SQRT_2, -consts::FRAC_1_SQRT_2).into()
    );

    let s = Sphere::default().with_transform(
        Mat4::identity()
            .rotate_z(consts::PI / 5.0)
            .scale((1., 0.5, 1.0).into()),
    );

    let n = s.normal_at((0., consts::FRAC_1_SQRT_2, -consts::FRAC_1_SQRT_2).into());
    assert_relative_eq!(n, (0.0, 0.97014, -0.24254).into());
}
