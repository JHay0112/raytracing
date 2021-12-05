/// Raytracer implemented in Rust
/// Guided by Raytracing in One Weekend, https://raytracing.github.io
/// 
/// Author: Jordan Hay
/// Date: 2021-12-04

// Inclusions

// Vectors
mod vec3;
pub use crate::vec3::Vec3;

// Main

fn main() {
    let mut v = Vec3{e: [1.0, 2.0, 3.0]};
    v[0] = 2.0;
    println!("{}", v[0]);
}