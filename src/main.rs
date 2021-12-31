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
use crate::shapes::triangle::{Triangle};
use crate::shapes::material;

use rand::Rng;
use rand::thread_rng;

// Functions

fn ray_color(r: &Ray, objects: &ShapeVec, depth: u8) -> Color {
    // Check depth isn't too high
    if (depth <= 0) {
        return Color::new(0.0, 0.0, 0.0);
    }
    // Check for a hit
    let intersection = objects.intersects(r, 0.001, f32::INFINITY);
    // Match it
    match intersection {
        Intersection::True{point, normal, t, material} => {
            // Get a scattered ray
            let scattered = material.scatter(r, &intersection);
            // Check for scatter
            match scattered {
                material::Scatter::False => {
                    // No scattering
                    // This case should never run
                    return Color::new(0.0, 0.0, 0.0);
                },
                material::Scatter::True{ray, attenuation} => {
                    // There is scattering
                    return *attenuation * ray_color(&ray, objects, depth - 1);
                }
            }
        },
        Intersection::False => {
            // No intersection, this is the background
            let unit_direction = vec3::normalize(r.direction);
            let t = 0.5 * (unit_direction.y() + 1.0);
            return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
        }
    }
}

// Main

fn main() {
    // Setup Scene
    let mut image = Image::new(16.0/9.0, 400);
    let samples = 30;
    let depth = 20;
    let camera = Camera::new(Point3::new(0.0, 0.0, 0.0), image.aspect_ratio, 2.0, 1.0);
    let mut objects = ShapeVec::new();

    // Create a lambertian material
    let mat1 = material::Lambertian::boxed(Color::new(0.0, 0.0, 1.0));
    let mat2 = material::Lambertian::boxed(Color::new(0.7, 0.0, 0.0));

    // Add objects
    objects.push(Sphere::boxed(Point3::new(0.0, -100.5, -1.0), 100.0, mat1));
    objects.push(Triangle::boxed(Point3::new(0.0, 0.25, -1.0), Point3::new(-0.8, -0.8, -1.5), Point3::new(-0.1, -1.0, -1.0), mat2.clone()));
    objects.push(Triangle::boxed(Point3::new(0.0, 0.25, -1.0), Point3::new(1.0, -1.0, -1.5), Point3::new(-0.1, -1.0, -1.0), mat2.clone()));

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
                let u: f32 = ((j as f32) + rng.gen_range(0.0..1.0))/((image.width - 1) as f32); // Proportion accross
                let v: f32 = ((i as f32) + rng.gen_range(0.0..1.0))/((image.height - 1) as f32); // Propotion down
                let r: Ray = camera.get_ray(u, v);
                // Let's see if a ray hits any objects
                pixel = pixel + ray_color(&r, &objects, depth);
            }
            // Normalize pixel, gamma correct and output it
            let scale = 1.0/(samples as f32);
            pixel[0] = f32::sqrt(pixel[0] * scale);
            pixel[1] = f32::sqrt(pixel[1] * scale);
            pixel[2] = f32::sqrt(pixel[2] * scale);

            image[i as usize][j as usize] = pixel;
        }
    }

    // Output image
    image.ppm("out.ppm");
}