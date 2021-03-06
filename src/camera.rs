/// Camera Management
/// 
/// Author: Jordan Hay
/// Date: 2021-12-07

// Inclusions

use crate::vec3::{Vec3, Point3};
use crate::image::Image;
use crate::ray::Ray;

// Classes

pub struct Camera {
    origin: Point3,
    viewport_height: f32,
    viewport_width: f32,
    focal_length: f32,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3
}

impl Camera {
    /// Instantiate a new Camera
    pub fn new(origin: Point3, aspect_ratio: f32, vh: f32, focal_length: f32) -> Self {

        // Derive unknowns
        // Viewport width
        let vw: f32 = aspect_ratio * vh;
        // Horizontal
        let hoz = Vec3::new(vw, 0.0, 0.0);
        // Vertical
        let vert = Vec3::new(0.0, vh, 0.0);
        // Lower left corner
        let llc = origin - hoz/2.0 - vert/2.0 - Vec3::new(0.0, 0.0, focal_length);

        return Self {
            origin: origin, 
            viewport_height: vh,
            viewport_width: vw,
            focal_length: focal_length,
            horizontal: hoz,
            vertical: vert,
            lower_left_corner: llc
        };
    }

    /// Creates a Ray emmited from the camera.
    /// 
    /// # Arguments
    /// 
    /// * `u` - The proportion across the image 
    /// * `v` - The proportion down the image
    pub fn get_ray(&self, u: f32, v:f32) -> Ray {
        return Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin);
    }

}