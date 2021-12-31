/// Triangle Object
/// 
/// Author: Jordan Hay
/// Date: 31/12/2021

// Inclusions

use crate::shapes::{Intersection, Shape};
use crate::shapes::material::{Material};
use crate::ray::{Ray};
use crate::vec3::{Vec3, Point3, angle_between, cross, dot};

// Classes

/// Defines a Triangle in 3D Space
///
/// # Attributes
/// 
/// `e` - Array of the three points in space that define the triangle. 
/// `material` - Box of material used by triangle.
pub struct Triangle<'a> {
    e: [Point3; 3],
    material: &'a dyn Material
}

impl<'a> Triangle<'a> {
    /// Construct a Triangle
    pub fn new(a: Point3, b: Point3, c: Point3, material: &'a dyn Material) -> Triangle<'a> {
        return Self{e: [a, b, c], material: material};
    }
    /// Construct a sphere in a box
    pub fn boxed(a: Point3, b: Point3, c: Point3, material: &'a dyn Material) -> Box<dyn Shape + 'a> {
        return Box::new(Self::new(a, b, c, material));
    }
}

/// Triangle and Ray Intersection
impl<'a> Shape for Triangle<'a> {
    fn intersects(&self, r: &Ray, min: f32, max: f32) -> Intersection {
        // Get the triangle plane normal
        let n: Vec3 = cross(self.e[1] - self.e[0], self.e[2] - self.e[0]);
        // Now find the point at which the ray intersects the triangle plane
        // Get the multiple of the ray direction 
        let t: f32 = dot(self.e[0] - r.origin, n)/dot(r.direction, n);
        // Check if it's within bounds
        if (t < min || max < t) {
            // If not then there's no intersection
            return Intersection::False;
        }
        // We're in bounds, let us check it lies on the triangle
        // Get the point
        let p: Point3 = r.at(t);
        // Checking with vertex zero as origin
        // In order to be in the triangle the angle between a-c must be less than a-b
        let a = self.e[1] - self.e[0];
        let b = self.e[2] - self.e[0];
        let c = p - self.e[0];
        if (angle_between(a, b) <= angle_between(a, c)) {
            return Intersection::False;
        }
        // Now do the same with vertex one as origin
        // Note that b is being used as the reference here instead of a
        // This prevents a bug caused by measuring from the same relative vector twice
        let a = self.e[0] - self.e[1];
        let b = self.e[2] - self.e[1];
        let c = p - self.e[1];
        if (angle_between(b, a) <= angle_between(b, c)) {
            return Intersection::False;
        }
        // And finally with vertex two
        let a = self.e[0] - self.e[2];
        let b = self.e[1] - self.e[2];
        let c = p - self.e[2];
        if (angle_between(a, b) <= angle_between(a, c)) {
            return Intersection::False;
        }

        // Else we survived all the tests so the point must be in the triangle
        // Record it and return intersection info
        return Intersection::True{
            point: p,
            normal: n,
            t: t,
            material: self.material
        };
    }
}