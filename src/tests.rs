use crate::vec3::{Point3, Vec3};
use approx::assert_relative_eq;

#[test]
fn add() {
    let a = Point3::new(3., -2., 5.);
    let b = Vec3::new(-2., 3., 1.);

    // adding vector to a point gives a point
    let c: Point3 = a + b;
    assert_relative_eq!(c, Point3::new(1., 1., 6.));

    let a = Vec3::new(3., -2., 5.);
    let b = Vec3::new(-2., 3., 1.);

    // adding two vectors gives a vector
    let c: Vec3 = a + b;
    assert_relative_eq!(c, Vec3::new(1., 1., 6.));

    // adding two points makes no sense
}

#[test]
fn sub() {
    let a = Point3::new(3., 2., 1.);
    let b = Point3::new(5., 6., 7.);

    // subbing two points gives a vector
    let c: Vec3 = a - b;
    assert_relative_eq!(c, Vec3::new(-2., -4., -6.));

    let a = Point3::new(3., 2., 1.);
    let b = Vec3::new(5., 6., 7.);

    // subbing vector from a point gives a point
    let c: Point3 = a - b;
    assert_relative_eq!(c, Point3::new(-2., -4., -6.));

    let a = Vec3::new(3., 2., 1.);
    let b = Vec3::new(5., 6., 7.);

    // subbing two vectors gives a vector
    let c: Vec3 = a - b;
    assert_relative_eq!(c, Vec3::new(-2., -4., -6.));

    // subbing point from vector makes no sense
}
