use std::f32::consts::PI;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use crate::errors::*;
use crate::vector::{vec3, Vec3};
use crate::world::World;

use serde::de::DeserializeOwned;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

#[derive(Debug, Copy, Clone)]
/// A resolution given by a width and a height.
pub struct Resolution {
    pub w: u32,
    pub h: u32,
}

#[derive(Clone, Copy)]
pub enum RealRange {
    /// A range that allows all values.
    All,
    /// A closed range that allows all values in \[a, b\], i.e. all x such that a <= x <= b.
    Closed(f32, f32),
    /// An open range that allows all values in (a, b), i.e. all x such that a < x < b.
    Open(f32, f32),
    /// A half-open range that allows all values in [a, b) i.e. all x such that a <= x < b.
    HalfOpenR(f32, f32),
    /// A half-open range that allows all values in (a, b] i.e. all x such that a < x <= b.
    HalfOpenL(f32, f32),
    /// A range that allows all values smaller than some a, i.e. all x such that x < a.
    SmallerThan(f32),
    /// A range that allows all values smaller than or equal to some a, i.e. all x such that x <= a.
    SmallerEqual(f32),
    /// A range that allows all values larger than some a, i.e. all x such that x > a.
    LargerThan(f32),
    /// A range that allows all values larger than or equal to some a, i.e. all x such that x >= a.
    LargerEqual(f32),
}

impl RealRange {
    pub fn contains(&self, x: f32) -> bool {
        match self.to_owned() {
            RealRange::All => true,
            RealRange::Closed(a, b) => a <= x && x <= b,
            RealRange::Open(a, b) => a < x && x < b,
            RealRange::HalfOpenR(a, b) => a <= x && x < b,
            RealRange::HalfOpenL(a, b) => a < x && x <= b,
            RealRange::SmallerThan(a) => x < a,
            RealRange::SmallerEqual(a) => x <= a,
            RealRange::LargerThan(a) => x > a,
            RealRange::LargerEqual(a) => x >= a,
        }
    }
}

pub fn move_pyramid(world: &mut World, by: Vec3) {
    if let Some(pyramid) = world.vertex_objects.get_mut(1) {
        pyramid.pos += by;
    }
}

pub fn print_frame_time(frame_time_ms: f32) {
    if frame_time_ms > 0.0000000001 {
        print!(
            "\rLast frame took {:.1} MS | {:.1} FPS",
            frame_time_ms,
            1000.0 / frame_time_ms
        );
        std::io::stdout().flush();
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

impl SphericalCoordinates {
    pub fn new(rad: f32, theta: f32, phi: f32) -> SphericalCoordinates {
        SphericalCoordinates { rad, theta, phi }
    }

    pub fn new_strict(
        rad: f32,
        theta: f32,
        phi: f32,
    ) -> Result<SphericalCoordinates, SphericalCreationError> {
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
        let rad =
            (cartesian.x * cartesian.x + cartesian.y * cartesian.y + cartesian.z * cartesian.z)
                .sqrt();
        let theta = (cartesian.z / rad).acos();
        let mut phi = (cartesian.y).atan2(cartesian.x);
        // Translate [-PI, PI] to [0, 2*PI]
        if phi < 0.0 {
            phi = 2.0 * PI + phi;
        }
        Self::new_strict(rad, theta, phi)
            .expect("These values can only ever be out of bounds due to a programmer's error.")
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

/// Write an object into a json file using Serde serialization.
pub fn save_object_as_file_json<O, P>(object: O, path: P) -> Result<(), Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
    O: Serialize,
{
    let file = File::create(path)?;
    serde_json::to_writer(file, &object)?;
    Ok(())
}
/// Create an object from a json file using Serde deserialization.
pub fn load_object_from_file_json<O, P>(path: P) -> Result<O, Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
    O: DeserializeOwned,
{
    // Reading from a string slice is much faster than using reading from a
    // File or BufReader using `serde_json::from_reader`
    let mut s = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut s)?;
    let result = serde_json::from_str(&s)?;
    Ok(result)
}
/// Write an object into a binary file using bincode/Serde serialization.
pub fn save_object_as_file_bin<O, P>(object: O, path: P) -> Result<(), Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
    O: Serialize,
{
    let file = File::create(path)?;
    let mut buf_writer = BufWriter::new(file);
    bincode::serialize_into(&mut buf_writer, &object)?;
    Ok(())
}
/// Create an object from a binary file using bincode/Serde deserialization.
pub fn load_object_from_file_bin<O, P>(path: P) -> Result<O, Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
    O: DeserializeOwned,
{
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    let result = bincode::deserialize_from(buf_reader)?;
    Ok(result)
}
