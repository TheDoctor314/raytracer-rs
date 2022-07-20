use approx::assert_relative_eq;

use crate::{ray::Ray, vec3::Point3};

#[test]
fn pos() {
    let r = Ray::new((2., 3., 4.), (1., 0., 0.));

    assert_relative_eq!(r.pos(0.), Point3::new(2., 3., 4.));
    assert_relative_eq!(r.pos(1.), Point3::new(3., 3., 4.));
    assert_relative_eq!(r.pos(-1.), Point3::new(1., 3., 4.));
    assert_relative_eq!(r.pos(2.5), Point3::new(4.5, 3., 4.));
}
