/// Objects in a Scene
/// 
/// Author: Jordan Hay
/// Date: 12/12/2021

// Inclusions

pub mod sphere;

use crate::ray::{Ray};
use crate::vec3::{Vec3, Point3};

// Enums

/// Describes the intersection of a ray and object
pub enum Intersection {
    /// Intersection
    /// 
    /// # Attributes
    /// 
    /// `point` - The point at which the intersection occured.
    /// `normal` - Normal vector of the surface at the point of intersection.
    /// `t` - The scalar applied to the ray's direction vector. A relative form of distance.
    True {
        point: Point3,
        normal: Vec3,
        t: f32
    },
    /// No Intersection
    False
}

// Traits

/// Intersection trait for objects
pub trait Shape {
    /// Determines if a Ray intersects with the object
    /// 
    /// # Arguments
    /// 
    /// `r` - Ray to check for intersection
    fn intersects(&self, r: &Ray, min: f32, max: f32) -> Intersection;
}

// Structs

/// Vector of Shapes
pub struct ShapeVec {
    e: Vec<Box<dyn Shape>>
}

impl ShapeVec {
    /// Initialise an empty shape vector
    pub fn new() -> Self {
        return Self {e: Vec::new()};
    }

    /// Push a (boxed) Shape onto the vector
    /// 
    /// # Arguments
    /// 
    /// `shape` - A boxed object that implements the Shape Trait.
    pub fn push(&mut self, shape: Box<dyn Shape>) {
        &self.e.push(shape);
    }

    /// Determines if a ray intersects with any shapes in the vector, returns the closest intersection
    /// 
    /// # Arguments
    /// 
    /// `r` - The ray to determine intersection with.
    pub fn intersects(&self, r: &Ray, min: f32, max: f32) -> Intersection {
        // Store closest record
        let closest: Intersection = Intersection::False;
        // Go through shapes in vector
        for shape in &self.e {
            // Create a hit record
            let record = shape.intersects(r, min, max);
            // Check whether there was a hit
            match record {
                Intersection::False => (),  // No intersection, nothing changes
                Intersection::True{t, ..} => { // There is an intersection
                    // Rename t to something else
                    let record_t = t;
                    // Check the state of the closest hit
                    match closest {
                        Intersection::False => {
                            // No closest record, replace it
                            let closest = record;
                        },
                        Intersection::True{t, ..} => { // There is a closest record
                            // Rename t to something else
                            let closest_t = t;
                            if (closest_t > record_t) {
                                // Closest is further than record so replace
                                let closest = record;
                            }
                        }
                    }
                }
            }
        }
        // Return the closest record
        return closest;
    }
}