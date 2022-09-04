use std::f32::consts::FRAC_1_SQRT_2;

use approx::assert_relative_eq;

use crate::{
    lights::PointLight,
    material::Material,
    vec3::{Point3, Vec3},
};

fn get_defaults() -> (Material, Point3) {
    (Material::default(), Point3::default())
}

#[test]
fn eye_between_light_and_surface() {
    let (m, pos) = get_defaults();
    let eyev = Vec3::new(0., 0., -1.);
    let normal = Vec3::new(0., 0., -1.);
    let light = PointLight::new((0., 0., -10.), [1., 1., 1.]);

    // all kinds of reflection at full strength so
    // color = ambient + diffuse + specular.
    let color = m.lighting(&light, pos, eyev, normal);
    assert_relative_eq!(color, [1.9, 1.9, 1.9].into());

    // eye is offset by 45 degrees.
    let eyev = Vec3::new(0., FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
    // here specular is effectively zero.
    let color = m.lighting(&light, pos, eyev, normal);
    assert_relative_eq!(color, [1.0, 1.0, 1.0].into());
}

#[test]
fn eye_opposite_surface() {
    // here, the light is offset by 45 degrees.

    let (m, pos) = get_defaults();
    let eyev = Vec3::new(0., 0., -1.);
    let normal = Vec3::new(0., 0., -1.);
    let light = PointLight::new((0., 10., -10.), [1., 1., 1.]);

    // specular = 0, diffuse = 0.9 * 1/(2.sqrt())
    let color = m.lighting(&light, pos, eyev, normal);
    let i = 0.7364;
    assert_relative_eq!(color, [i, i, i].into());
}

#[test]
fn eye_and_light_offset_by_45() {
    let (m, pos) = get_defaults();
    let eyev = Vec3::new(0., -FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
    let normal = Vec3::new(0., 0., -1.);
    let light = PointLight::new((0., 10., -10.), [1., 1., 1.]);

    // specular = 0.9, diffuse = 0.9 * 1/(2.sqrt())
    let color = m.lighting(&light, pos, eyev, normal);
    let i = 1.6364;
    assert_relative_eq!(color, [i, i, i].into());
}

#[test]
fn light_behind_surface() {
    let (m, pos) = get_defaults();
    let eyev = Vec3::new(0., 0., -1.);
    let normal = Vec3::new(0., 0., -1.);
    let light = PointLight::new((0., 0., 10.), [1., 1., 1.]);

    // only ambient since light is behind the surface.
    let color = m.lighting(&light, pos, eyev, normal);
    assert_relative_eq!(color, [0.1, 0.1, 0.1].into());
}
