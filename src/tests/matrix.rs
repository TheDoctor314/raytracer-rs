use crate::{
    matrix::Mat4,
    vec3::{Point3, Vec3},
};
use approx::assert_relative_eq;

#[test]
fn new() {
    let mat: Mat4 = [
        [1.0, 2.0, 3.0, 4.0],
        [5.5, 6.5, 7.5, 8.5],
        [9., 10., 11., 12.],
        [13.5, 14.5, 15.5, 16.5],
    ]
    .into();

    assert_relative_eq!(mat[(0, 0)], 1.);
    assert_relative_eq!(mat[(0, 3)], 4.);
    assert_relative_eq!(mat[(1, 0)], 5.5);
    assert_relative_eq!(mat[(1, 2)], 7.5);
    assert_relative_eq!(mat[(2, 2)], 11.);
    assert_relative_eq!(mat[(3, 0)], 13.5);
    assert_relative_eq!(mat[(3, 2)], 15.5);
}

#[test]
fn mul() {
    let a: Mat4 = [
        [1., 2., 3., 4.],
        [5., 6., 7., 8.],
        [9., 8., 7., 6.],
        [5., 4., 3., 2.],
    ]
    .into();

    let b: Mat4 = [
        [-2., 1., 2., 3.],
        [3., 2., 1., -1.],
        [4., 3., 6., 5.],
        [1., 2., 7., 8.],
    ]
    .into();

    assert_relative_eq!(
        &a * &b,
        [
            [20., 22., 50., 48.],
            [44., 54., 114., 108.],
            [40., 58., 110., 102.],
            [16., 26., 46., 42.],
        ]
        .into()
    );

    let a: Mat4 = [
        [1., 2., 3., 4.],
        [2., 4., 4., 2.],
        [8., 6., 4., 1.],
        [0., 0., 0., 1.],
    ]
    .into();

    let b = Vec3::new(1., 2., 3.);

    assert_relative_eq!(&a * b, Vec3::new(14., 22., 32.));

    let b = Point3::new(1., 2., 3.);
    assert_relative_eq!(&a * b, Point3::new(18., 24., 33.));
}

#[test]
fn identity() {
    let a: Mat4 = [
        [0., 1., 2., 4.],
        [1., 2., 4., 8.],
        [2., 4., 8., 16.],
        [4., 8., 16., 32.],
    ]
    .into();

    assert_relative_eq!(&a * &Mat4::identity(), a);
}

#[test]
fn transpose() {
    let mut a: Mat4 = [
        [0., 9., 3., 0.],
        [9., 8., 0., 8.],
        [1., 8., 5., 3.],
        [0., 0., 5., 8.],
    ]
    .into();

    a.transposed();

    assert_relative_eq!(
        a,
        [
            [0., 9., 1., 0.],
            [9., 8., 8., 0.],
            [3., 0., 5., 5.],
            [0., 8., 3., 8.],
        ]
        .into()
    );

    assert_relative_eq!(Mat4::identity().transpose(), Mat4::identity());
}

#[test]
fn determinant() {
    assert_relative_eq!(Mat4::identity().determinant(), 1.0);

    let mat: Mat4 = [
        [-2., -8., 3., 5.],
        [-3., 1., 7., 3.],
        [1., 2., -9., 6.],
        [-6., 7., 7., -9.],
    ]
    .into();

    assert_relative_eq!(mat.determinant(), -4071.);

    let mat: Mat4 = [
        [6., 4., 4., 4.],
        [5., 5., 7., 6.],
        [4., -9., 3., -7.],
        [9., 1., 7., -6.],
    ]
    .into();

    assert_relative_eq!(mat.determinant(), -2120.);

    let mat: Mat4 = [
        [-4., 2., -2., -3.],
        [9., 6., 2., 6.],
        [0., -5., 1., -5.],
        [0., 0., 0., 0.],
    ]
    .into();

    assert_relative_eq!(mat.determinant(), 0.);
}

#[test]
fn inverse() {
    let mat: Mat4 = [
        [-5., 2., 6., -8.],
        [1., -5., 1., 8.],
        [7., 7., -6., -7.],
        [1., -3., 7., 4.],
    ]
    .into();

    assert_relative_eq!(mat.determinant(), 532.);

    assert_relative_eq!(
        mat.inverse().unwrap(),
        [
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]
        .into()
    );

    let mat: Mat4 = [
        [8., -5., 9., 2.],
        [7., 5., 6., 1.],
        [-6., 0., 9., 6.],
        [-3., 0., -9., -4.],
    ]
    .into();

    assert_relative_eq!(
        mat.inverse().unwrap(),
        [
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]
        .into()
    );

    let mat: Mat4 = [
        [9., 3., 0., 9.],
        [-5., -2., -6., -3.],
        [-4., 9., 6., 4.],
        [-7., 6., 6., 2.],
    ]
    .into();

    assert_relative_eq!(
        mat.inverse().unwrap(),
        [
            [-0.04074, -0.07778, 0.1444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333]
        ]
        .into()
    );

    // uninvertible
    let mat: Mat4 = [
        [-4., 2., -2., -3.],
        [9., 6., 2., 6.],
        [0., -5., 1., -5.],
        [0., 0., 0., 0.],
    ]
    .into();

    assert!(mat.inverse().is_none());

    let a: Mat4 = [
        [3., -9., 7., 3.],
        [3., -8., 2., -9.],
        [-4., 4., 4., 1.],
        [-6., 5., -1., 1.],
    ]
    .into();

    let b: Mat4 = [
        [8., 2., 2., 2.],
        [3., -1., 7., 0.],
        [7., 0., 5., 4.],
        [6., -2., 0., 5.],
    ]
    .into();

    let c = &a * &b;
    assert_relative_eq!(&c * &b.inverse().unwrap(), a);
}

#[test]
fn translation() {
    let transform = Mat4::new_translation((5., -3., 2.).into());
    let inv = transform.inverse().unwrap();

    let p = Point3::new(-3., 4., 5.);
    let v = Vec3::new(-3.0, 4.0, 5.0);

    assert_relative_eq!(&transform * p, (2., 1., 7.).into());
    assert_relative_eq!(&inv * p, (-8., 7., 3.).into());

    assert_relative_eq!(&transform * v, v);
}

#[test]
fn scaling() {
    let transform = Mat4::new_scaling((2., 3., 4.).into());
    let inv = transform.inverse().unwrap();

    let p = Point3::new(-4., 6., 8.);
    let v = Vec3::new(-4., 6., 8.);

    assert_relative_eq!(&transform * p, (-8., 18., 32.).into());
    assert_relative_eq!(&transform * v, (-8., 18., 32.).into());

    assert_relative_eq!(&inv * v, (-2., 2., 2.).into());

    let transform = Mat4::new_scaling((-1., 1., 1.).into());
    assert_relative_eq!(&transform * Point3::new(2., 3., 4.), (-2., 3., 4.).into());
}

#[test]
fn rotation() {
    use std::f32::consts;

    // around the x-axis
    let p: Point3 = (0., 1., 0.).into();
    let half_quarter = Mat4::new_rotation_x(consts::FRAC_PI_4);
    let full_quarter = Mat4::new_rotation_x(consts::FRAC_PI_2);

    assert_relative_eq!(
        &half_quarter * p,
        (0., 2.0f32.sqrt().recip(), 2.0f32.sqrt().recip()).into()
    );
    assert_relative_eq!(
        &half_quarter.inverse().unwrap() * p,
        (0., 2.0f32.sqrt().recip(), -(2.0f32.sqrt().recip())).into()
    );

    assert_relative_eq!(&full_quarter * p, (0., 0., 1.).into());

    // around the y-axis
    let p: Point3 = (0., 0., 1.).into();
    let half_quarter = Mat4::new_rotation_y(consts::FRAC_PI_4);
    let full_quarter = Mat4::new_rotation_y(consts::FRAC_PI_2);

    assert_relative_eq!(
        &half_quarter * p,
        (2.0f32.sqrt().recip(), 0., 2.0f32.sqrt().recip()).into()
    );

    assert_relative_eq!(&full_quarter * p, (1., 0., 0.).into());

    // around the z-axis
    let p: Point3 = (0., 1., 0.).into();
    let half_quarter = Mat4::new_rotation_z(consts::FRAC_PI_4);
    let full_quarter = Mat4::new_rotation_z(consts::FRAC_PI_2);

    assert_relative_eq!(
        &half_quarter * p,
        (-(2.0f32.sqrt().recip()), 2.0f32.sqrt().recip(), 0.).into()
    );

    assert_relative_eq!(&full_quarter * p, (-1., 0., 0.).into());
}

#[test]
fn shearing() {
    let transform = Mat4::new_shearing(1., 0., 0., 0., 0., 0.);
    let p: Point3 = (2., 3., 4.).into();

    assert_relative_eq!(&transform * p, (5., 3., 4.).into());

    let transform = Mat4::new_shearing(0., 1., 0., 0., 0., 0.);
    assert_relative_eq!(&transform * p, (6., 3., 4.).into());

    let transform = Mat4::new_shearing(0., 0., 1., 0., 0., 0.);
    assert_relative_eq!(&transform * p, (2., 5., 4.).into());

    let transform = Mat4::new_shearing(0., 0., 0., 1., 0., 0.);
    assert_relative_eq!(&transform * p, (2., 7., 4.).into());

    let transform = Mat4::new_shearing(0., 0., 0., 0., 1., 0.);
    assert_relative_eq!(&transform * p, (2., 3., 6.).into());

    let transform = Mat4::new_shearing(0., 0., 0., 0., 0., 1.);
    assert_relative_eq!(&transform * p, (2., 3., 7.).into());
}

#[test]
fn chained_transforms() {
    let p: Point3 = (1., 0., 1.).into();

    let a = Mat4::new_rotation_x(std::f32::consts::FRAC_PI_2);
    let b = Mat4::new_scaling((5., 5., 5.).into());
    let c = Mat4::new_translation((10., 5., 7.).into());

    let p2 = &a * p;
    assert_relative_eq!(p2, (1., -1., 0.).into());

    let p3 = &b * p2;
    assert_relative_eq!(p3, (5., -5., 0.).into());

    let p4 = &c * p3;
    assert_relative_eq!(p4, (15., 0., 7.).into());

    let transform = &c * &(&b * &a);
    assert_relative_eq!(&transform * p, p4);

    let transform = Mat4::identity()
        .rotate_x(std::f32::consts::FRAC_PI_2)
        .scale((5., 5., 5.).into())
        .translate((10., 5., 7.).into());

    assert_relative_eq!(&transform * p, p4);
}

#[test]
fn default_view() {
    let from = Point3::default();
    let to = Point3::new(0., 0., -1.);
    let up = Vec3::new(0., 1., 0.);

    assert_relative_eq!(Mat4::view_transform(from, to, up), Mat4::identity());
}

#[test]
fn view_looking_toward_positive_z() {
    let from = Point3::default();
    let to = Point3::new(0., 0., 1.);
    let up = Vec3::new(0., 1., 0.);

    assert_relative_eq!(
        Mat4::view_transform(from, to, up),
        Mat4::new_scaling((-1., 1., -1.).into())
    );
}

#[test]
fn view_transform_moves_the_world() {
    let from = Point3::new(0., 0., 8.);
    let to = Point3::new(0., 0., 0.);
    let up = Vec3::new(0., 1., 0.);

    assert_relative_eq!(
        Mat4::view_transform(from, to, up),
        Mat4::new_translation((0., 0., -8.).into())
    );
}

#[test]
fn arbitrary_view_transform() {
    let from = Point3::new(1., 3., 2.);
    let to = Point3::new(4., -2., 8.);
    let up = Vec3::new(1., 1., 0.);

    assert_relative_eq!(
        Mat4::view_transform(from, to, up),
        [
            [-0.50709, 0.50709, 0.67612, -2.36643],
            [0.76772, 0.60609, 0.12122, -2.82843],
            [-0.35857, 0.59761, -0.71714, 0.0],
            [0., 0., 0., 1.]
        ]
        .into()
    );
}
