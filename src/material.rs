//! This module describes the material properties of the objects in
//! the scene. We use the Phong reflection model here.

use crate::{
    lights::PointLight,
    vec3::{Point3, Vec3},
    Color,
};

/// Describes the properties of the material.
#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    /// The color of the object.
    pub(crate) color: Color,
    /// Background lighting of the environment.
    pub(crate) ambient: f32,
    /// Light reflected from a matte surface. Depends only on the angle
    /// between the light source and the surface normal.
    pub(crate) diffuse: f32,
    /// Reflection of the light source itself. Depends on the angle between
    /// the reflection and the eye vector.
    pub(crate) specular: f32,
    /// Controls the specular highlight.
    pub(crate) shininess: f32,
}

impl Material {
    /// Construct a new `Material`.
    pub fn new(color: Color, ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    /// Responsible for shading the point based on the material.
    pub fn lighting(&self, light: &PointLight, pos: Point3, eyev: Vec3, normal: Vec3) -> Color {
        let effective_color = self.color.blend(light.intensity);
        let lightv = (light.pos - pos).normalize();

        let ambient = effective_color * self.ambient;
        let light_dot_normal = lightv.dot(normal);

        let (diffuse, specular) = if light_dot_normal < 0.0 {
            (Color::BLACK, Color::BLACK)
        } else {
            let diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflectv = (-lightv).reflect(normal);
            let reflect_dot_eye = reflectv.dot(eyev);

            let specular = if reflect_dot_eye <= 0.0 {
                Color::BLACK
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                light.intensity * self.specular * factor
            };

            (diffuse, specular)
        };

        ambient + diffuse + specular
    }

    /// Sets the color.
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    /// Sets the color.
    pub fn with_ambient(mut self, ambient: f32) -> Self {
        self.ambient = ambient;
        self
    }
    /// Sets the color.
    pub fn with_diffuse(mut self, diffuse: f32) -> Self {
        self.diffuse = diffuse;
        self
    }
    /// Sets the color.
    pub fn with_specular(mut self, specular: f32) -> Self {
        self.specular = specular;
        self
    }
    /// Sets the color.
    pub fn with_shininess(mut self, shininess: f32) -> Self {
        self.shininess = shininess;
        self
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: [1., 1., 1.].into(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}
