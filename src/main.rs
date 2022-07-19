use raytracer_rs::{matrix::Mat4, vec3::Point3};

fn main() {
    let width = 800;
    let height = 800;
    let mut canvas = image::Rgb32FImage::new(width, height);
    let pixel: image::Rgb<f32> = [1.0, 0.0, 0.0].into();

    let center = ((width / 2) as f32, (height / 2) as f32);
    let radius = (width as f32) / 4.0;
    let convert_coords = |x: f32, z: f32| -> (u32, u32) {
        let x = x * radius;
        let y = z * radius;

        let (x, y) = (x + center.0, y + center.1);
        (x as u32, y as u32)
    };

    let mut hours = 12;

    let mut p: Point3 = (0., 0., 1.).into();
    let transform = Mat4::new_rotation_y(std::f32::consts::FRAC_PI_6);

    while hours > 0 {
        let (x, y) = convert_coords(p.x(), p.z());
        canvas.put_pixel(x, y, pixel);

        p = &transform * p;
        hours -= 1;
    }

    let canvas = image::DynamicImage::ImageRgb32F(canvas).to_rgb8();

    canvas.save("clock.png").unwrap();
}
