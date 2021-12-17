/// Objects in a Scene
/// 
/// Author: Jordan Hay
/// Date: 12/12/2021

// Inclusions

#[path = "../ray.rs"]
mod ray;
use crate::ray::Ray;

#[path = "../vec3.rs"]
mod vec3;
use crate::vec3::{Vec3, Point3};

#[path = "material.rs"]
mod material;
use crate::material

// Enums

/// Describes the intersection of a ray and object
pub enum Intersection {
    /// Intersection
    /// 
    /// # Attributes
    /// 
    /// `point` - The point at which the intersection occured.
    /// `normal` - Normal vector of the surface at the point of intersection.
    /// `material` - Reference to the material
    True {
        point: Point3,
        normal: Vec3,
        material: &Material
    },
    /// No Intersection
    False
}

// Traits

/// Intersection trait for objects
pub trait Intersects {
    /// Determines if a Ray intersects with the object
    /// 
    /// # Arguments
    /// 
    /// `r` - Ray to check for intersection
    pub fn intersects(&self, &r: Ray, min: f32 = 0, max: f32 = f32::INFINITY) -> Intersection;
}

// Aliases

pub type ShapeVec = Vec<Box<dyn Intersects>>;