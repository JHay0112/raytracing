#![allow(dead_code)]

/// Image
/// 
/// Author: Jordan Hay
/// Date: 2021-12-05

// Inclusions

// mod vec3; // Current not working, strange really
use crate::vec3::{Color};
use std::ops;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

// Classes

/// Image
/// 
/// Stores pixels, info and functions for output.
pub struct Image {
    pub height: u16,
    pub width: u16,
    pub aspect_ratio: f32,
    pixels: Vec<Vec<Color>>
}

impl Image {
    /// Initialise a new image
    pub fn new(aspect_ratio: f32, width: u16) -> Self {
        // Calculate height
        let height: u16 = ((width as f32)/aspect_ratio) as u16;
        // Create pixels vector
        let mut pixels: Vec<Vec<Color>> = Vec::new();
        // Insert empty colours
        for _ in 0..height {
            let mut v: Vec<Color> = Vec::new();
            for _ in 0..width {
                v.push(Color::new(0.0, 0.0, 0.0));
            }
            pixels.push(v);
        }
        
        return Self{height: height, width: width, aspect_ratio: aspect_ratio, pixels: pixels};
    }

    /// Output a PPM File
    pub fn ppm(&self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path)?;
        let mut string = format!("P3\n{} {}\n255\n", self.width, self.height);
        // Top to bottom
        for i in (0..self.height).rev() {
            // Left to right
            for j in 0..self.width {
                string += &format!("{}\n", self.pixels[i as usize][j as usize]);
            }
        }
        file.write_all(string.as_bytes())?;
        return Ok(());
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