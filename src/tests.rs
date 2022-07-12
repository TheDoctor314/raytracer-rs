use crate::vec3::{Point3, Vec3};
use approx::assert_relative_eq;

#[test]
fn xyz() {
    let a = Vec3::new(3., -2., 5.);

    assert_relative_eq!(a.x(), 3.);
    assert_relative_eq!(a.y(), -2.);
    assert_relative_eq!(a.z(), 5.);
}
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

#[test]
fn neg() {
    let v = Vec3::new(1., -2., 3.);
    assert_relative_eq!(-v, Vec3::new(-1., 2., -3.));
}

#[test]
fn mul_div() {
    let v = Vec3::new(1., -2., 3.);

    assert_relative_eq!(v * 3.5,  Vec3::new(3.5, -7., 10.5));
    assert_relative_eq!(v * 0.5,  Vec3::new(0.5, -1., 1.5));

    assert_relative_eq!(v / 2.,  Vec3::new(0.5, -1., 1.5));
}

#[test]
fn magnitude() {
    assert_relative_eq!(Vec3::new(0., 1., 0.).mag(), 1.);
    assert_relative_eq!(Vec3::new(0., 0., 1.).mag(), 1.);
    assert_relative_eq!(Vec3::new(1., 2., 3.).mag(), 14f32.sqrt());
    assert_relative_eq!(Vec3::new(-1., -2., -3.).mag(), 14f32.sqrt());
}

#[test]
fn normalize() {
    assert_relative_eq!(Vec3::new(4.0, 0.0, 0.0).normalize(), Vec3::new(1.0, 0., 0.));
    assert_relative_eq!(Vec3::new(1.0, 2.0, 3.0).normalize(), Vec3::new(1.0 / 14f32.sqrt(), 2. / 14f32.sqrt(), 3. / 14f32.sqrt()));
}

#[test]
fn dot() {
    let a = Vec3::new(1.0, 2., 3.);
    let b = Vec3::new(2., 3., 4.);

    assert_relative_eq!(a.dot(b), 20f32);
}

#[test]
fn cross() {
    let a = Vec3::new(1., 2., 3.);
    let b = Vec3::new(2., 3., 4.);

    assert_relative_eq!(a.cross(b), Vec3::new(-1., 2., -1.));
    assert_relative_eq!(b.cross(a), Vec3::new(1., -2., 1.));
}