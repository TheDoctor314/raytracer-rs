use approx::assert_relative_eq;

use crate::{
    hit_list::HitRec, lights::PointLight, material::Material, matrix::Mat4, ray::Ray,
    sphere::Sphere, world::World, Color,
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

#[test]
fn shading() {
    let world = default_world();
    let ray = Ray::new((0., 0., -5.), (0., 0., 1.));

    let s = world.objects.first().unwrap();
    let hit = HitRec { t: 4.0, obj: s };

    let comps = hit.prepare_computations(&ray);
    let color = world.shade_hit(comps);

    assert_relative_eq!(color, [0.38066, 0.47583, 0.2855].into());
}

#[test]
fn shading_from_inside() {
    let mut world = default_world();
    if let Some(light) = world.lights.first_mut() {
        *light = PointLight::new((0., 0.25, 0.), [1., 1., 1.]);
    }

    let ray = Ray::new((0., 0., 0.), (0., 0., 1.));

    let s = &world.objects[1];
    let hit = HitRec { t: 0.5, obj: s };

    let comps = hit.prepare_computations(&ray);
    let color = world.shade_hit(comps);

    assert_relative_eq!(color, [0.90498, 0.90498, 0.90498].into());
}

#[test]
fn color_when_ray_misses() {
    let w = default_world();
    let r = Ray::new((0., 0., -5.), (0., 1., 0.));
    let color = w.color_at(&r);

    assert_relative_eq!(color, Color::BLACK);
}

#[test]
fn color_when_ray_hits() {
    let w = default_world();
    let r = Ray::new((0., 0., -5.), (0., 0., 1.));
    let color = w.color_at(&r);

    assert_relative_eq!(color, [0.38066, 0.47583, 0.2855].into());
}

#[test]
fn color_when_intersection_behind_ray() {
    let mut w = default_world();
    for obj in w.objects.iter_mut() {
        obj.material_mut().ambient = 1.0;
    }

    let r = Ray::new((0., 0., 0.75), (0., 0., -1.));

    let inner = &w.objects[1];
    assert_relative_eq!(w.color_at(&r), inner.material().color);
}
