/// 3D Vector Test
/// 
/// Author: Jordan Hay
/// Date: 10/12/2021

// Inclusions

#[path = "../src/vec3.rs"]
mod vec3;
use crate::vec3::{Vec3};

// Tests

/// Test Vector Addition
#[test]
fn test_add() {
    let v = Vec3::new(1.0, 2.0, 2.0);
    let u = Vec3::new(4.0, 0.0, -3.0);
    assert_eq!(v + u, Vec3::new(5.0, 2.0, -1.0));
}