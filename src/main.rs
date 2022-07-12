use raytracer_rs::vec3::{Point3, Vec3};

fn main() {
    let width = 900;
    let height = 500;
    let mut canvas = image::Rgb32FImage::new(width, height);
    let pixel: image::Rgb<f32> = [1.0, 0.0, 0.0].into();

    let env = Environment {
        gravity: Vec3::new(0.0, -0.1, 0.0),
        wind: Vec3::new(-0.01, 0.0, 0.0),
    };

    let mut proj = Projectile {
        pos: Point3::new(0.0, 1.0, 0.0),
        vel: Vec3::new(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let convert_coords = |x: f32, y: f32| {
        let new_y = height as f32 - y;
        let new_x = x as u32;

        (new_x, new_y as u32)
    };

    while proj.pos.y() > 0.0 {
        let (x, y) = (proj.pos.x(), proj.pos.y());
        let (x, y) = convert_coords(x, y);

        canvas.put_pixel(x, y, pixel);

        let new_proj = tick(&env, &proj);
        proj = new_proj;
    }

    let canvas = image::DynamicImage::ImageRgb32F(canvas);
    let canvas = canvas.to_rgb8();

    canvas.save("trajectory.png").unwrap();
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let pos = proj.pos + proj.vel;
    let vel = proj.vel + env.gravity + env.wind;

    Projectile { pos, vel }
}

#[derive(Debug)]
struct Projectile {
    pos: Point3,
    vel: Vec3,
}

#[derive(Debug)]
struct Environment {
    gravity: Vec3,
    wind: Vec3,
}
