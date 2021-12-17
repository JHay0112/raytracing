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
use crate::shapes::sphere::{Sphere};

// Main

fn main() {
    let image = Image::new(16.0/9.0, 1920);
    let camera = Camera::new(Point3::new(0.0, 0.0, 0.0), &image, 2.0, 1.0);
}