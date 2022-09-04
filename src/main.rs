use raytracer_rs::{ray::Ray, sphere::Sphere, vec3::Point3};

fn main() {
    let width = 400;
    let height = 400;
    let mut canvas = image::Rgb32FImage::new(width, height);
    let pixel: image::Rgb<f32> = [1.0, 0.0, 0.0].into();

    let wall_z = 10.0;
    let wall_size = 7.0;
    let half_wall = wall_size / 2.0;

    let pixel_size = wall_size / width as f32;
    let s = Sphere::new();
    let ray_origin = Point3::new(0.0, 0.0, -5.0);

    for y in 0..width {
        let world_y = half_wall - pixel_size * y as f32;

        for x in 0..height {
            let world_x = -half_wall + pixel_size * x as f32;
            let pos = Point3::new(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (pos - ray_origin).normalize());

            let mut xs = s.intersect(&ray);

            if xs.hit().is_some() {
                canvas.put_pixel(x, y, pixel)
            }
        }
    }

    let canvas = image::DynamicImage::ImageRgb32F(canvas).to_rgb8();

    canvas.save("ch5.png").unwrap();
}
