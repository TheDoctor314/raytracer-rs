use crate::{
    lights::PointLight, material::Material, matrix::Mat4, ray::Ray, sphere::Sphere, world::World,
};

fn default_world() -> World {
    let light = PointLight::new((-10., 10., -10.), [1., 1., 1.]);
    let s1 = Sphere::new(
        Material::default()
            .with_color([0.8, 1.0, 0.6].into())
            .with_diffuse(0.7)
            .with_specular(0.2),
    );

    let s2 = Sphere::default().with_transform(Mat4::new_scaling((0.5, 0.5, 0.5).into()));

    World::new(vec![s1, s2], vec![light])
}

#[test]
fn intersect_world() {
    let world = default_world();
    let ray = Ray::new((0., 0., -5.), (0., 0., 1.));

    let xs = world.intersect(&ray);

    assert_eq!(xs.len(), 4);
    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 4.5);
    assert_eq!(xs[2].t, 5.5);
    assert_eq!(xs[3].t, 6.0);
}
