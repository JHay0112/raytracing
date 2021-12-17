/// Sphere Object
/// 
/// Author: Jordan Hay
/// Date: 17/12/2021

// Inclusions

mod objects;
use crate::objects::{Intersection, Intersects};

mod ray;
use crate::ray::{Ray};

mod vec3;
use crate::vec3::{Vec3, Point3};

// Classes

/// Defines a Sphere in 3D Space
/// 
/// # Attributes
/// 
/// `origin` - The central point of the sphere.
/// `radius` - The radius of the sphere.
pub struct Sphere {
    origin: Point3,
    radius: f32
}

/// Sphere and Ray Intersection
impl Intersects for Sphere {
    pub fn intersects(&self, &r: Ray, min: f32 = 0, max: f32 = f32::INFINITY) -> Intersection {
        // Calculating Ray-Sphere Quadratic Intersection Equation

        // Get distance between origins
        let separation: Vec3 = r.origin() - self.origin();
        // at^2 + bt + c = 0
        let a: f32 = vec3::dot(r.direction(), r.direction()); // If a point is on the sphere then this == (radius)^2
        let half_b: f32 = vec3::dot(separation, r.direction()); 
        let c: f32 = f32::sqrt(separation.magnitude()) - self.radius * self.radius;
        // Discriminant of quadratic
        let discriminant = half_b * half_b - a * c;
        
        // If the discriminant is less than zero
        // There are no roots, so there is no intersection
        if (discriminant < 0) {
            return Intersection::False;
        }

        // Else we have roots! Are they in acceptable ranges?
        // First get the discriminant sqrt
        let sqrtd: f32 = f32::sqrt(discriminant);
        let root: f32 = (-half_b - sqrtd) / a;
        // If this is outside the range
        if (root < min || max < root) {
            // Then check the orther root
            root = (-half_b + sqrtd) / a;
            // If this is also outside the range then there is no intersection!
            if (root < min || max < root) {
                return Intersection::False;
            }
        }

        // Else we can record a hit!
        let intersection: Intersection = Intersection::True{
            point: r.at(root),
            normal: (root - self.center) / self.radius
        };

        return intersection;
    }
}