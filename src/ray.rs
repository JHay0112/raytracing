#![allow(dead_code)]

/// Rays + Associated Functions
/// 
/// Author: Jordan Hay
/// Date: 2021-12-05

// Inclusions

#[path = "vec3.rs"]
mod vec3;
use crate::vec3::{Vec3, Point3};

// Classes

/// Rays
/// 
/// Lines defined by an origin and a direction vector
pub struct Ray {
    origin: Point3,
    direction: Vec3
}

impl Ray {
    /// Initialise a new ray
    pub fn new(o: Point3, d: Vec3) -> Self {
        return Self{origin: o, direction: d};
    }

    /// Compute the point on the ray at t
    pub fn at(&self, t: f32) -> Point3 {
        return self.origin + t * self.direction;
    }
}

// Aliases

pub type Line = Ray;