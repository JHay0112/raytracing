/// Raytracer implemented in Rust
/// Guided by Raytracing in One Weekend, https://raytracing.github.io
/// 
/// Author: Jordan Hay
/// Date: 2021-12-04

// Inclusions

// Vectors
mod vec3;
pub use crate::vec3::{Vec3, Color, Point3};

// Main

fn main() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let u = Vec3::new(0.0, 1.0, 0.0);
    println!("{}", v + u);
    println!("{}", vec3::dot(u, v));
}