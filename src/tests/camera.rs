use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4};

use approx::assert_relative_eq;

use crate::{
    camera::Camera,
    matrix::Mat4,
    vec3::{Point3, Vec3},
    Color,
};

use super::world::default_world;

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

#[test]
fn render_default_world() {
    // very basic test for the expected colour at the center of the canvas.
    let world = default_world();
    let transform = {
        let from = Point3::new(0., 0., -5.);
        let to = Point3::default();
        let up = Vec3::new(0., 1., 0.);
        Mat4::view_transform(from, to, up)
    };
    let cam = Camera::new(11, 11, FRAC_PI_2).with_transform(transform);

    let image = cam.render(&world);
    let pixel = *image.get_pixel(5, 5);
    let color = Color(pixel);

    assert_relative_eq!(
        color,
        Color(image::Rgb::<f32>([0.38066, 0.475826, 0.28549]))
    );
}
