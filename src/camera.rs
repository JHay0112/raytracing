/// Camera Management
/// 
/// Author: Jordan Hay
/// Date: 2021-12-07

// Inclusions

mod vec3;
use crate::vec3::{Vec3, Point3};
mod image;
use crate::image::Image;

// Classes

pub struct Camera {
    position: Point3,
    orientation: Vec3,
    image: Image
}

impl Camera {
    /// Instantiate a new Camera
    pub fn new(position: Point3, orientation: Vec3, image: Image) -> Self {
        return Self{position: position, orientation: orientation, image: image};
    }


}