use std::f32::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};

use raytracer_rs::{
    camera::Camera, lights::PointLight, material::Material, matrix::Mat4, sphere::Sphere,
    world::World,
};

fn main() {
    let mut args = std::env::args();
    args.next();

    let file = args.next().expect("Output file name expected");

    let floor = Sphere::new(
        Material::default()
            .with_color([1., 0.9, 0.9].into())
            .with_specular(0.0),
    )
    .with_transform(Mat4::new_scaling((10., 0.01, 10.0).into()));

    let left_wall = Sphere::new(floor.material().clone()).with_transform(
        Mat4::new_scaling((10., 0.01, 10.).into())
            .rotate_x(FRAC_PI_2)
            .rotate_y(-FRAC_PI_4)
            .translate((0., 0., 5.).into()),
    );
    let right_wall = Sphere::new(floor.material().clone()).with_transform(
        Mat4::new_scaling((10., 0.01, 10.).into())
            .rotate_x(FRAC_PI_2)
            .rotate_y(FRAC_PI_4)
            .translate((0., 0., 5.).into()),
    );

    let middle = Sphere::new(
        Material::default()
            .with_color([0.1, 1., 0.5].into())
            .with_diffuse(0.7)
            .with_specular(0.3),
    )
    .with_transform(Mat4::new_translation((-0.5, 1., 0.5).into()));

    let right = Sphere::new(
        Material::default()
            .with_color([0.5, 1., 0.1].into())
            .with_diffuse(0.7)
            .with_specular(0.3),
    )
    .with_transform(Mat4::new_scaling((0.5, 0.5, 0.5).into()).translate((1.5, 0.5, -0.5).into()));

    let left = Sphere::new(
        Material::default()
            .with_color([1., 0.8, 0.1].into())
            .with_diffuse(0.7)
            .with_specular(0.3),
    )
    .with_transform(
        Mat4::new_scaling((0.333, 0.333, 0.333).into()).translate((-1.5, 0.333, -0.75).into()),
    );

    let light = PointLight::new((-10., 10., -10.), [1., 1., 1.]);
    let world = World::new(
        vec![floor, left_wall, right_wall, middle, right, left],
        vec![light],
    );

    let camera = Camera::new(400, 200, FRAC_PI_3).with_transform(Mat4::view_transform(
        (0., 1.5, -5.).into(),
        (0., 1., 0.).into(),
        (0., 1., 0.).into(),
    ));

    let canvas = camera.render(&world);
    let canvas = image::DynamicImage::ImageRgb32F(canvas).to_rgb8();

    canvas.save(&file).unwrap();
}
