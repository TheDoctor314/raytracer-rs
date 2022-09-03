use crate::{
    hit_list::{HitList, HitRec},
    ray::Ray,
    sphere::Sphere,
};

#[test]
fn ray_intersect() {
    let r = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
    let s = Sphere::new();

    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);

    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 6.0);

    assert!(std::ptr::eq(xs[0].obj, &s));
}

#[test]
fn ray_intersect_tangent() {
    let r = Ray::new((0.0, 1.0, -5.0), (0.0, 0.0, 1.0));
    let s = Sphere::new();

    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);

    assert_eq!(xs[0].t, 5.0);
    assert_eq!(xs[1].t, 5.0);
}

#[test]
fn ray_miss() {
    let r = Ray::new((0.0, 2.0, -5.0), (0.0, 0.0, 1.0));
    let s = Sphere::new();

    let xs = s.intersect(&r);
    assert!(xs.is_empty());
}

#[test]
fn ray_in_sphere() {
    let r = Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
    let s = Sphere::new();

    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);

    assert_eq!(xs[0].t, -1.0);
    assert_eq!(xs[1].t, 1.0);
}

#[test]
fn ray_behind_sphere() {
    let r = Ray::new((0.0, 0.0, 5.0), (0.0, 0.0, 1.0));
    let s = Sphere::new();

    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);

    assert_eq!(xs[0].t, -6.0);
    assert_eq!(xs[1].t, -4.0);
}

#[test]
fn hit_with_positive_t() {
    let s = Sphere::new();
    let mut xs = HitList::new(vec![HitRec { t: 1.0, obj: &s }, HitRec { t: 2.0, obj: &s }]);

    assert_eq!(xs.hit(), Some(&HitRec { t: 1.0, obj: &s }));
}

#[test]
fn hit_with_negative_t() {
    {
        let s = Sphere::new();
        let mut xs = HitList::new(vec![
            HitRec { t: -1.0, obj: &s },
            HitRec { t: 1.0, obj: &s },
        ]);

        assert_eq!(xs.hit(), Some(&HitRec { t: 1.0, obj: &s }));
    }

    {
        let s = Sphere::new();
        let mut xs = HitList::new(vec![
            HitRec { t: -2.0, obj: &s },
            HitRec { t: -1.0, obj: &s },
        ]);

        assert_eq!(xs.hit(), None);
    }
}

#[test]
fn hit_with_t_in_random_order() {
    let s = Sphere::new();
    let mut xs = HitList::new(vec![
        HitRec { t: 5.0, obj: &s },
        HitRec { t: 7.0, obj: &s },
        HitRec { t: -3.0, obj: &s },
        HitRec { t: 2.0, obj: &s },
    ]);

    assert_eq!(xs.hit(), Some(&HitRec { t: 2.0, obj: &s }));
}
