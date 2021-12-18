#![allow(unused)]

/// Raytracer implemented in Rust
/// Guided by Raytracing in One Weekend, https://raytracing.github.io
/// 
/// Author: Jordan Hay
/// Date: 2021-12-04

// Inclusions

mod vec3;
use crate::vec3::{Vec3, Color, Point3};

mod image;
use crate::image::{Image};

mod ray;
use crate::ray::{Ray};

mod camera;
use crate::camera::{Camera};

mod shapes;
use crate::shapes::{ShapeVec, Intersection};
use crate::shapes::sphere::{Sphere};

use rand::prelude::*;

// Main

fn main() {
    // Setup Scene
    let image = Image::new(16.0/9.0, 400);
    let samples = 50;
    let camera = Camera::new(Point3::new(0.0, 0.0, 0.0), &image, 2.0, 1.0);
    let mut objects = ShapeVec::new();

    // Add objects
    objects.push(Sphere::boxed(Point3::new(0.0, 0.0, -2.0), 1.0));

    // Random number generator
    let mut rng = rand::thread_rng();

    // Top to bottom
    for i in (0..image.height).rev() {
        // Left to right
        for j in (0..image.width) {
            // Initialise a pixel
            let mut pixel = Color::new(0.0, 0.0, 0.0);
            // For samples
            for s in (0..samples) {
                // Create a ray with some random jitter
                let u: f32 = ((j as f32) + rng.gen::<f32>())/((image.width - 1) as f32); // Proportion accross
                let v: f32 = ((i as f32) + rng.gen::<f32>())/((image.height - 1) as f32); // Propotion down
                let r: Ray = camera.get_ray(u, v);
                // Let's see if a ray hits any objects
                let intersection = objects.intersects(&r, 0.0, f32::INFINITY);
            }
        }
    }

    // Output image
    image.ppm("test.ppm");
}