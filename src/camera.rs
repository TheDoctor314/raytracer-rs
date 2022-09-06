//! Defines a camera which allows us to `take pictures` of the scene.
//!
//! The canvas of the camera is always one unit in front of it.
//! This makes the math cleaner.

use crate::{matrix::Mat4, ray::Ray, vec3::Point3};

/// The `Camera` allows us to look at a scene and render it.
#[derive(Debug)]
pub struct Camera {
    /// Width of the canvas in pixels.
    hsize: u32,
    /// Height of the canvas in pixels.
    vsize: u32,
    /// Field of view in radians.
    /// Describes how much the camera can see. When `fov` is small, the
    /// camera is zoomed in on a smaller area of the scene.
    fov: f32,
    /// A view transform describing how the world should be oriented
    /// relative to the camera
    transform: Mat4,
    /// Inverse of the transform.
    transform_inv: Mat4,
    /// Size of the pixels in world-space.
    pixel_size: f32,
    /// Width of half of the canvas in world-space.
    half_width: f32,
    /// Height of half of the canvas in world-space.
    half_height: f32,
}

impl Camera {
    /// Constructs a `Camera`. The default transform is the identity matrix.
    pub fn new(hsize: u32, vsize: u32, fov: f32) -> Self {
        let (pixel_size, half_width, half_height) = {
            let half_view = (fov / 2.0).tan();
            let aspect_ratio = hsize as f32 / vsize as f32;

            let (hw, hh) = if aspect_ratio >= 1.0 {
                (half_view, half_view / aspect_ratio)
            } else {
                (half_view * aspect_ratio, half_view)
            };

            let ps = (hw * 2.0) / hsize as f32;
            (ps, hw, hh)
        };

        Self {
            hsize,
            vsize,
            fov,
            transform: Mat4::identity(),
            transform_inv: Mat4::identity(),
            pixel_size,
            half_width,
            half_height,
        }
    }

    /// Sets the transform for the camera.
    pub fn with_transform(mut self, transform: Mat4) -> Self {
        self.transform_inv = transform.inverse().unwrap_or_else(Mat4::identity);
        self.transform = transform;
        self
    }

    /// Returns a new ray that starts at the camera and passes through the
    /// given pixel on the canvas.
    pub fn ray_for_pixel(&self, x: u32, y: u32) -> Ray {
        let x_offset = (x as f32 + 0.5) * self.pixel_size;
        let y_offset = (y as f32 + 0.5) * self.pixel_size;

        // camera looks toward -z, so +x is to the left
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        // canvas is at z = -1
        let pixel = &self.transform_inv * Point3::new(world_x, world_y, -1.);
        let orig = &self.transform_inv * Point3::default();
        let dir = (pixel - orig).normalize();

        Ray::new(orig, dir)
    }
}
