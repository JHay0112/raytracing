#![allow(dead_code)]

/// Image
/// 
/// Author: Jordan Hay
/// Date: 2021-12-05

// Inclusions

use std::ops;
use crate::vec3::Color;

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
        return Self{height: ((width as f32)/aspect_ratio) as u16, width: width, aspect_ratio: aspect_ratio, pixels: Vec::new()};
    }

    /// Output a PPM File
    pub fn ppm(filename: String) {

    }
}

/// Access Image rows with Indexing
impl ops::Index<usize> for Image {
    type Output = Vec<Color>;
    fn index(&self, i: usize) -> &Self::Output {
        return &self.pixels[i];
    }
}

/// Modify Image rows with Indexing
impl ops::IndexMut<usize> for Image {
    fn index_mut(&mut self, i: usize) -> &mut Vec<Color> {
        return &mut self.pixels[i];
    }
}