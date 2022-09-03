//! Defines all the unit tests for this library.
//! They are in a separate module so that the whole library
//! is not recompiled for running tests.

mod matrix;
mod ray;
mod vec3;
mod sphere;

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
