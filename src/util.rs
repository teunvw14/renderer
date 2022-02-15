use std::f32::consts::PI;

use crate::vector::{Vec3, vec3};
use crate::world::World;

#[derive(Debug, Copy, Clone)]
/// A resolution given by a width and a height.
pub struct Resolution {
    pub w: u32,
    pub h: u32,
}

#[derive(Clone, Copy)]
pub enum RealRange {
    All,
    Between(f32, f32),
    SmallerThan(f32),
    LargerThan(f32),
}

pub fn in_real_range(range: RealRange, num: f32) -> bool {
    match range {
        RealRange::All => true,
        RealRange::Between(a, b) => a < num && num < b,
        RealRange::SmallerThan(a) => num < a,
        RealRange::LargerThan(a) => num > a,
    }
}

pub fn move_triangle(world: &mut World, by: Vec3) {
    if let Some(triangle) = world.objects.get_mut(0) {
        triangle.pos += by;
    }
}

pub fn print_frame_time(frame_time_ms: f32) {
    if frame_time_ms > 0.05 {
        print!(
            "\rLast frame took {:.1} MS | {:.1} FPS",
            frame_time_ms,
            1000.0 / frame_time_ms
        );
    }
}

/// Spherical coordinates, where theta represents the angle counter-clockwise
/// from the positive z-axis and phi is the counter-clockwise rotation from the
/// positive x-axis.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SphericalCoordinates {
    pub rad: f32, 
    pub theta: f32,
    pub phi: f32,
}

quick_error! {
    #[derive(Debug)]
    pub enum SphericalCreationError {
        NegativeRadius {
            display("Negative radius is invalid.")
        }
        ThetaOutOfBounds {
            display("Theta out of legal range [0, PI]")
        }
        PhiOutOfBounds {
            display("Phi out of legal range [0, 2*PI]")
        }
    }
}

impl SphericalCoordinates {
    pub fn new(rad: f32, theta: f32, phi: f32) -> SphericalCoordinates{
        SphericalCoordinates {
            rad,
            theta,
            phi
        }
    }

    pub fn new_strict(rad: f32, theta: f32, phi: f32) -> Result<SphericalCoordinates, SphericalCreationError> {
        if rad < 0.0 {
            return Err(SphericalCreationError::NegativeRadius);
        }
        if theta < 0.0 || theta > PI {
            return Err(SphericalCreationError::ThetaOutOfBounds);
        }
        if phi < 0.0 || phi > 2.0 * PI {
            return Err(SphericalCreationError::PhiOutOfBounds);
        }
        Ok(Self::new(rad, theta, phi))
    }
}

impl From<Vec3> for SphericalCoordinates {
    fn from(cartesian: Vec3) -> Self {
        let rad = (
            cartesian.x * cartesian.x +
            cartesian.y * cartesian.y + 
            cartesian.z * cartesian.z
        ).sqrt();
        let theta = (cartesian.z / rad).acos();
        let phi = (cartesian.y).atan2(cartesian.x);
        Self {
            rad,
            theta,
            phi
        }
    }
}

#[test]
fn test_vec_to_sphere_conversion() {
    let v = vec3(1.0, 1.0, 0.0);
    let s = SphericalCoordinates {
        rad: (2f32).sqrt(), 
        theta: PI / 2.0,
        phi: PI / 4.0,
    };
    assert_eq!(s, v.into());
}