#![allow(dead_code)]

/// Image
/// 
/// Author: Jordan Hay
/// Date: 2021-12-05

// Inclusions

use vec3;
pub use crate::vec3::{Color};

// Classes

/// Image
/// 
/// Stores pixels, info and functions for output.
pub struct Image {
    height: u16,
    width: u16,
    aspect_ratio: f32,
    pixels: Vec<Vec<Color>>
}

impl Image {
    /// Initialise a new image
    pub fn new(aspect_ratio: f32, width: u16) -> Self {
        return Self{height: width/aspect_ratio, width: width, aspect_ratio: aspect_ratio};
    }

    /// Output a PPM File
    pub fn ppm(filename: str) {

    }
}