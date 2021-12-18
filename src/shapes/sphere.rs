/// Sphere Object
/// 
/// Author: Jordan Hay
/// Date: 17/12/2021

// Inclusions

use crate::shapes::{Intersection, Shape};
use crate::ray::{Ray};
use crate::vec3::{Vec3, Point3, dot};

// Classes

/// Defines a Sphere in 3D Space
/// 
/// # Attributes
/// 
/// `origin` - The central point of the sphere.
/// `radius` - The radius of the sphere.
pub struct Sphere {
    pub origin: Point3,
    pub radius: f32
}

impl Sphere {
    /// Construct a sphere
    pub fn new(origin: Point3, radius: f32) -> Self {
        return Self{origin: origin, radius: radius};
    }

    /// Construct a sphere in a Box<dyn Intersects>
    pub fn boxed(origin: Point3, radius: f32) -> Box<dyn Shape> {
        return Box::new(Self::new(origin, radius));
    }
}

/// Sphere and Ray Intersection
impl Shape for Sphere {
    fn intersects(&self, r: &Ray, min: f32, max: f32) -> Intersection {
        // Calculating Ray-Sphere Quadratic Intersection Equation

        // Get distance between origins
        let separation: Vec3 = r.origin - self.origin;
        // at^2 + bt + c = 0
        let a: f32 = dot(r.direction, r.direction); // If a point is on the sphere then this == (radius)^2
        let half_b: f32 = dot(separation, r.direction); 
        let c: f32 = f32::sqrt(separation.magnitude()) - self.radius * self.radius;
        // Discriminant of quadratic
        let discriminant = half_b * half_b - a * c;
        
        // If the discriminant is less than zero
        // There are no roots, so there is no intersection
        if (discriminant < 0.0) {
            return Intersection::False;
        }

        // Else we have roots! Are they in acceptable ranges?
        // First get the discriminant sqrt
        let sqrtd: f32 = f32::sqrt(discriminant);
        let root: f32 = (-half_b - sqrtd) / a;
        // If this is outside the range
        if (root < min || max < root) {
            // Then check the orther root
            let root: f32 = (-half_b + sqrtd) / a;
            // If this is also outside the range then there is no intersection!
            if (root < min || max < root) {
                return Intersection::False;
            }
        }

        // Else we can record a hit!
        let intersection: Intersection = Intersection::True {
            point: r.at(root),
            normal: (r.at(root) - self.origin) / self.radius,
            t: root
        };

        return intersection;
    }
}