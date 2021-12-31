#![allow(dead_code)]

/// 3D Vectors + Associated Functions
/// 
/// Author: Jordan Hay
/// Date: 2021-12-04

// Inclusions

use std::ops;
use std::fmt;
use std::cmp;
use rand::Rng;
use rand::thread_rng;

// Classes

/// 3D Vectors for Linearalgebra in R3
/// 
/// # Example
/// 
/// ```
/// let mut v = Vec3::new(1.0, 2.0, 3.0);
/// v[0] = 9.0;
/// assert_eq!(9.0, v[0]);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    e: [f32; 3]
}

/// Standard Functions for Vec3
impl Vec3 {
    /// Initalise a vector (The nice way)
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        return Self{e: [x, y, z]};
    }

    // Vector Cartesian Component Aliases
    pub const fn x(self) -> f32 {return self.e[0];}
    pub const fn y(self) -> f32 {return self.e[1];}
    pub const fn z(self) -> f32 {return self.e[2];}
    // Vector Alphabetical Component Aliases
    pub const fn a(self) -> f32 {return self.e[0];}
    pub const fn b(self) -> f32 {return self.e[1];}
    pub const fn c(self) -> f32 {return self.e[2];}

    // Vector Attributes
    /// Magnitude of vector
    pub fn magnitude(&self) -> f32 {
        return (self[0].powf(2.0) + self[1].powf(2.0) + self[2].powf(2.0)).sqrt();
    }

    /// Length of vector (alias of Vec3::magnitude())
    pub fn length(&self) -> f32 {
        return self.magnitude();
    }

    /// Vector with random values in range
    pub fn random(min: f32, max: f32) -> Self {
        // Setup RNG
        let mut rng = thread_rng();
        // Create and return vec3
        return Self::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max));
    }

    /// Checks if Vector is near zero
    pub fn near_zero(&self) -> bool {
        const S: f32 = 1e-8;
        return (f32::abs(self[0]) < S) && (f32::abs(self[1]) < S) && (f32::abs(self[2]) < S);
    }
}

/// Default Vector
impl Default for Vec3 {
    fn default() -> Self {
        return Self::new(0.0, 0.0, 0.0);
    }
}

/// Vector Display
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{} {} {}", self[0], self[1], self[2]);
    }
}

/// Vector Addition
impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        return Self::new(self[0] + other[0], self[1] + other[1], self[2] + other[2]);
    }
}

/// Negative Vectors
impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        return Self::new(-self[0], -self[1], -self[2]);
    }
}

/// Vector Subtraction
impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Vec3) -> Self::Output {
        return Self::new(self[0] - other[0], self[1] - other[1], self[2] - other[2]);
    }
}

/// Vector Itemwise Multiplication
impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, other: Vec3) -> Self::Output {
        return Self::new(self[0] * other[0], self[1] * other[1], self[2] * other[2]);
    }
}

/// Vector Scalar Multiplication
impl ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self::Output {
        return Self::new(self[0] * scalar, self[1] * scalar, self[2] * scalar);
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, vector: Vec3) -> Self::Output {
        return Vec3::new(vector[0] * self, vector[1] * self, vector[2] * self);
    }
}

/// Vector Scalar Division
impl ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, scalar: f32) -> Self::Output {
        return Self::new(self[0] / scalar, self[1] / scalar, self[2] / scalar);
    }
}

/// Vector Equality
impl cmp::PartialEq for Vec3 {
    fn eq(&self, &u: &Self) -> bool {
        return self[0] == u[0] && self[1] == u[1] && self[2] == u[2];
    }
}

/// Access Vector Components with Index
impl ops::Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, i: usize) -> &Self::Output {
        return &self.e[i];
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        return &mut self.e[i];
    }
}

// Functions

/// Vector Dot Product
pub fn dot(u: Vec3, v: Vec3) -> f32 {
    return u[0] * v[0] + u[1] * v[1] + u[2] * v[2];
}

/// Vector Cross Product
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    return Vec3::new(u[1] * v[2] - u[2] * v[1],
                     u[2] * v[0] - u[0] * v[2],
                     u[0] * v[1] - u[1] * v[0]);
}

/// Normalize Vector
pub fn normalize(u: Vec3) -> Vec3 {
    let magnitude = u.magnitude();
    return Vec3::new(u[0]/magnitude, u[1]/magnitude, u[2]/magnitude);
}

/// Angle between two vectors
pub fn angle_between(u: Vec3, v: Vec3) -> f32 {
    return (dot(u, v)/(u.magnitude() * v.magnitude())).acos();
}

// Random Functions

/// Produces a random unit vector in a unit sphere.
pub fn random_unit_sphere_vector() -> Vec3 {
    // Setup random number generator
    let rng = rand::thread_rng();
    let mut p: Vec3;
    // Continue producing until we get a valid vector
    loop {
        // Create random vector in unit cube
        p = Vec3::random(-1.0, 1.0);
        if (f32::powf(p.length(), 2.0) >= 1.0) {
            // Outside sphere
            continue; // Go round again
        }
        // In unit sphere
        break; // Let's use it
    }
    return normalize(p); // Return normalized vector
}

// Aliases

pub type Point3 = Vec3;
pub type Color = Vec3;