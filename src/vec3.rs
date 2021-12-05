/// 3D Vectors + Associated Functions
/// 
/// Author: Jordan Hay
/// Date: 2021-12-04

// Inclusions

use std::ops;

// Classes

/// 3D Vector
pub struct Vec3 {
    pub e: [f32; 3]
}

/// Standard Functions for Vec3
impl Vec3 {
    // Vector Component Aliases
    pub fn x(self) -> f32 {return self.e[0];}
    pub fn y(self) -> f32 {return self.e[1];}
    pub fn z(self) -> f32 {return self.e[2];}
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