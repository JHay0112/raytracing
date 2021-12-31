/// Materials for Shapes
/// 
/// Author: Jordan Hay
/// Date: 19/12/2021

// Inclusions

use crate::ray::{Ray};
use crate::vec3::{Color, random_unit_sphere_vector};
use crate::shapes::{Intersection};

use std::rc::{Rc};

// Enums

pub enum Scatter<'a> {
    /// There is no scattering
    False, 
    /// A ray was scattered
    /// 
    /// # Attributes
    /// 
    /// `ray` - The scattered ray
    /// `attentuation` - The colour from the ray
    True {
        ray: Ray,
        attenuation: &'a Color
    }
}

// Traits

/// Material Trait for Shapes
pub trait Material {
    /// Computes a ray scattered by the material
    /// 
    /// # Attributes
    /// 
    /// `ray_in` - The incoming ray. 
    /// `intersection` - Description of ray and object intersection. 
    fn scatter(&self, ray_in: &Ray, intersection: &Intersection) -> Scatter;
}

// Classes with Material Trait

/// Lambertian/Diffuse materials
/// 
/// # Attributes
/// 
/// `albedo` - Color of the material.
pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        return Self{albedo: albedo};
    }
    /// Create a boxed material
    pub fn boxed(albedo: Color) -> Rc<dyn Material> {
        return Rc::new(Lambertian{albedo: albedo});
    }
}

/// Lambertian Scattering
impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, intersection: &Intersection) -> Scatter {
        match intersection {
            Intersection::True {point, normal, t, material} => {
                // Produce a randomised scatter direction
                let mut scatter_direction = *normal + random_unit_sphere_vector();
                // Sometimes this gives really small vectors, we don't want these
                if (scatter_direction.near_zero()) {
                    scatter_direction = *normal;
                }
                // Return scatter ray
                return Scatter::True{ray: Ray::new(*point, scatter_direction), attenuation: &self.albedo};
            },
            Intersection::False => {
                return Scatter::False;
            }
        }
    }
}