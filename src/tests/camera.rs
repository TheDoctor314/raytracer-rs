use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4};

use approx::assert_relative_eq;

use crate::{camera::Camera, matrix::Mat4};

#[test]
fn ray_through_center_of_canvas() {
    let cam = Camera::new(201, 101, FRAC_PI_2);
    let r = cam.ray_for_pixel(100, 50);

    assert_relative_eq!(r.orig, (0.0, 0.0, 0.0).into());
    assert_relative_eq!(r.dir, (0., 0., -1.).into());
}

#[test]
fn ray_through_corner_of_canvas() {
    let cam = Camera::new(201, 101, FRAC_PI_2);
    let r = cam.ray_for_pixel(0, 0);

    assert_relative_eq!(r.orig, (0.0, 0.0, 0.0).into());
    assert_relative_eq!(r.dir, (0.66519, 0.33259, -0.66851).into());
}

#[test]
fn ray_through_transformed_camera() {
    let cam = Camera::new(201, 101, FRAC_PI_2).with_transform(
        Mat4::new_rotation_y(FRAC_PI_4) * &Mat4::new_translation((0., -2., 5.).into()),
    );

    let r = cam.ray_for_pixel(100, 50);

    assert_relative_eq!(r.orig, (0.0, 2.0, -5.0).into());
    assert_relative_eq!(r.dir, (FRAC_1_SQRT_2, 0., -FRAC_1_SQRT_2).into());
}
