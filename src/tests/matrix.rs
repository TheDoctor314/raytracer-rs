use crate::{matrix::Mat4, vec3::Vec3};
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
