#![allow(dead_code)]

/// Rays + Associated Functions
/// 
/// Author: Jordan Hay
/// Date: 2021-12-05

// Inclusions

use vec3;
pub use crate::vec3::{Vec3, Point3, Color};

// Classes

/// Rays
/// 
/// Lines defined by an origin and a direction vector
pub struct Ray {
    origin Vec3;
    direction Vec3;
}

impl Ray {
    /// Initialise a new ray
    fn new(origin: Point3, direction Vec3) -> Self {
        return Self{origin: origin, direction: vec3::normalize(direction)};
    }

    /// Compute the point on the ray at t
    fn at(t: f32) -> Point3 {
        return self.origin + t * self.direction;
    }
}

// Aliases

type Line = Ray;