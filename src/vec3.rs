/// 3D Vectors + Associated Functions
/// 
/// Author: Jordan Hay
/// Date: 2021-12-04

// Inclusions

use std::ops;
use std::fmt;

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
pub struct Vec3 {
    pub e: [f32; 3]
}

/// Standard Functions for Vec3
impl Vec3 {
    /// Initalise a vector (The nice way)
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        return Self{e: [x, y, z]};
    }

    // Vector Component Aliases
    pub fn x(self) -> f32 {return self.e[0];}
    pub fn y(self) -> f32 {return self.e[1];}
    pub fn z(self) -> f32 {return self.e[2];}
    pub fn a(self) -> f32 {return self.e[0];}
    pub fn b(self) -> f32 {return self.e[1];}
    pub fn c(self) -> f32 {return self.e[2];}
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
        return write!(f, "({}, {}, {})", self[0], self[1], self[2]);
    }
}

/// Vector Addition
impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        return Vec3::new(self[0] + other[0], self[1] + other[1], self[2] + other[2]);
    }
}

/// Access Vector Components with Index
impl ops::Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, i: usize) -> &Self::Output {
        return &self.e[i];
    }
}

/// Mutate Vector Components with Index
impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        return &mut self.e[i];
    }
}