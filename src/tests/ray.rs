use approx::assert_relative_eq;

use crate::{matrix::Mat4, ray::Ray, vec3::Point3};

#[test]
fn pos() {
    let r = Ray::new((2., 3., 4.), (1., 0., 0.));

    assert_relative_eq!(r.pos(0.), Point3::new(2., 3., 4.));
    assert_relative_eq!(r.pos(1.), Point3::new(3., 3., 4.));
    assert_relative_eq!(r.pos(-1.), Point3::new(1., 3., 4.));
    assert_relative_eq!(r.pos(2.5), Point3::new(4.5, 3., 4.));
}

#[test]
fn transform() {
    let r = Ray::new((1., 2., 3.), (0., 1., 0.));
    let m = Mat4::new_translation((3., 4., 5.).into());

    let r2 = r.transform(&m);

    assert_relative_eq!(r2.orig, (4., 6., 8.).into());
    assert_relative_eq!(r2.dir, (0., 1., 0.).into());

    let m = Mat4::new_scaling((2., 3., 4.).into());

    let r2 = r.transform(&m);

    assert_relative_eq!(r2.orig, (2., 6., 12.).into());
    assert_relative_eq!(r2.dir, (0., 3., 0.).into());
}
