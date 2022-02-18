use std::{
    f32::consts::PI,
    ops::{Add, AddAssign, Mul, Sub},
};

use serde::{Deserialize, Serialize};

use crate::util::SphericalCoordinates;

pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

#[derive(Serialize, Deserialize, Clone, Copy, Default, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

/// Dot-product
impl Mul for Vec3 {
    type Output = f32;

    fn mul(self, other: Self) -> Self::Output {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        vec3(rhs * self.x, rhs * self.y, rhs * self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        vec3(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        vec3(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl From<SphericalCoordinates> for Vec3 {
    fn from(sphere: SphericalCoordinates) -> Self {
        let x = sphere.rad * (sphere.phi).cos() * (sphere.theta).sin();
        let y = sphere.rad * (sphere.phi).sin() * (sphere.theta).sin();
        let z = sphere.rad * (sphere.theta).cos();
        vec3(x, y, z)
    }
}

// Custom implementations
impl Vec3 {
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&mut self) {
        *self = *self * (1.0 / self.len())
    }

    pub fn normalized(&self) -> Vec3 {
        *self * (1.0 / self.len())
    }

    pub fn cross_product(&self, other: Self) -> Vec3 {
        vec3(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn distance_to(&self, other: Self) -> f32 {
        let diff_vector = self.clone() - other;
        diff_vector.len()
    }

    /// Set the length of a vector while keeping the direction.
    pub fn set_length(&mut self, length: f32) {
        self.normalize();
        *self = *self * length;
    }

    /// Rotate around the x-axis, starting from the positive z-axis.
    pub fn rotate_x_rad(&mut self, angle: f32) {
        let current_angle = self.y.atan2(self.z);
        let new_angle = current_angle + angle;
        let len_yz = (self.y * self.y + self.z * self.z).sqrt();
        self.z = new_angle.cos() * len_yz;
        self.y = new_angle.sin() * len_yz;
    }
    /// Rotate around the y-axis, starting from the positive x-axis.
    pub fn rotate_y_rad(&mut self, angle: f32) {
        let current_angle = self.z.atan2(self.x);
        let new_angle = current_angle + angle;
        let len_xz = (self.x * self.x + self.z * self.z).sqrt();
        self.x = new_angle.cos() * len_xz;
        self.z = new_angle.sin() * len_xz;
    }
    /// Rotate around the z-axis, starting from the positive y-axis.
    pub fn rotate_z_rad(&mut self, angle: f32) {
        let current_angle = self.y.atan2(self.x);
        let new_angle = current_angle + angle;
        let len_xy = (self.x * self.x + self.y * self.y).sqrt();
        self.x = new_angle.cos() * len_xy;
        self.y = new_angle.sin() * len_xy;
    }
}

#[test]
fn test_sphere_to_vec_conversion() {
    let v = vec3(1.0, 1.0, 0.0);
    let s = SphericalCoordinates {
        rad: (2f32).sqrt(),
        theta: PI / 2.0,
        phi: PI / 4.0,
    };
    let epsilon = 0.000001f32;
    assert!((v - s.into()).len() < epsilon);
}
