use raytracer_rs::{
    lights::PointLight, material::Material, ray::Ray, sphere::Sphere, vec3::Point3,
};

fn main() {
    let mut args = std::env::args();
    args.next();

    let file = args.next().expect("Output file name expected");

    let width = 400;
    let height = 400;
    let mut canvas = image::Rgb32FImage::new(width, height);

    let wall_z = 10.0;
    let wall_size = 7.0;
    let half_wall = wall_size / 2.0;

    let pixel_size = wall_size / width as f32;

    let material = Material::default().with_color([1., 0.2, 1.].into());
    let s = Sphere::new(material);

    let light = PointLight::new((-10., 10., -10.), [1., 1., 1.]);

    let ray_origin = Point3::new(0.0, 0.0, -5.0);

    for y in 0..width {
        let world_y = half_wall - pixel_size * y as f32;

        for x in 0..height {
            let world_x = -half_wall + pixel_size * x as f32;
            let pos = Point3::new(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (pos - ray_origin).normalize());

            let mut xs = s.intersect(&ray);

            if let Some(hit) = xs.hit() {
                let point = ray.pos(hit.t);
                let normal = hit.obj.normal_at(point);
                let eye = -*ray.dir();

                let color = hit.obj.material().lighting(&light, point, eye, normal);

                canvas.put_pixel(x, y, color.into_inner());
            }
        }
    }

    let canvas = image::DynamicImage::ImageRgb32F(canvas).to_rgb8();

    canvas.save(&file).unwrap();
}
