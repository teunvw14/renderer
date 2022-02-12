use crate::vector::Vec3;

use std::io::{BufReader, Read, BufWriter};
use std::ops::{Deref, DerefMut};
use std::fs::{File};
use std::path::Path;
use serde::de::DeserializeOwned;
use serde::{Serialize, Serializer, Deserialize, ser::SerializeStruct};

use rgb::*;

/// Material that can be put on an object.
#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct Material {
    pub ambient_constant: RGBA8,
    pub diffuse_constant: f32,
    pub specular_constant: f32,
    pub shine: f32,
}

/// A ball object.
#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct Ball {
    pub pos: Vec3,
    pub rad: f32,
    pub material: Material,
}

#[derive(Debug, Clone, Copy)]
pub struct LightIntensity {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl LightIntensity {
    pub fn new(r: f32, g: f32, b: f32) -> LightIntensity {
        LightIntensity { r, g, b }
    }
}

#[derive(Debug)]
pub struct Light {
    pub pos: Vec3,
    pub diffuse_intensity: LightIntensity,
    pub specular_intensity: LightIntensity,
}

pub type FaceIndices = (usize, usize, usize);
pub type TriangleFace = (Vec3, Vec3, Vec3);

pub fn get_triangle_normal(triangle_face: TriangleFace) -> Vec3 {
    let (v0, v1, v2) = triangle_face;
    let v0v1 = v1 - v0;
    let v0v2 = v2 - v0;

    v0v1.cross_product(v0v2)
}

#[derive(Serialize, Deserialize)]
pub struct VertexObject {
    pub pos: Vec3,
    pub vertices: Vec<Vec3>,
    pub faces: Vec<FaceIndices>,
    pub material: Material,
}

impl VertexObject {
    pub fn iter_faces(&self) -> FacesIterator {
        FacesIterator::from_vertex_object(self)
    }
}

pub struct FacesIterator<'a> {
    index: usize,
    vertices: &'a Vec<Vec3>,
    face_indices: &'a Vec<FaceIndices>,
}

impl<'a> FacesIterator<'a> {
    pub fn from_vertex_object(vertex_object: &'a VertexObject) -> Self {
        FacesIterator {
            index: 0,
            vertices: &vertex_object.vertices,
            face_indices: &vertex_object.faces,
        }
    }
}

impl<'a> Iterator for FacesIterator<'a> {
    type Item = TriangleFace;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.face_indices.len() {
            let (index_v0, index_v1, index_v2) =
                self.face_indices.get(self.index).unwrap().to_owned();
            let v0 = self.vertices.get(index_v0).unwrap().to_owned();
            let v1 = self.vertices.get(index_v1).unwrap().to_owned();
            let v2 = self.vertices.get(index_v2).unwrap().to_owned();
            self.index += 1;
            Some((v0, v1, v2))
        } else {
            None
        }
    }
}

pub trait Object {
    fn pos(&self) -> Vec3;
    fn material(&self) -> Material;
    /// Write an object into a json file using Serde serialization.
    fn into_file_json<P>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> where 
    P: AsRef<Path>,
    Self: Serialize,
    {
        let file = File::create(path)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }
    /// Create an object from a json file using Serde deserialization.
    fn from_file_json<P>(path: P) -> Result<Self, Box<dyn std::error::Error>> where 
    P: AsRef<Path>,
    Self: DeserializeOwned,
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
    fn into_file_bin<P>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> where 
    P: AsRef<Path>,
    Self: Serialize,
    {
        let file = File::create(path)?;
        let mut buf_writer = BufWriter::new(file);
        bincode::serialize_into(&mut buf_writer, self)?;
        Ok(())
    }
    /// Create an object from a binary file using bincode/Serde deserialization.
    fn from_file_bin<P>(path: P) -> Result<Self, Box<dyn std::error::Error>> where 
    P: AsRef<Path>,
    Self: DeserializeOwned,
    {
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);
        let result = bincode::deserialize_from(buf_reader)?;
        Ok(result)
    }
}

impl Object for VertexObject {
    fn pos(&self) -> Vec3 {
        self.pos
    }
    fn material(&self) -> Material {
        self.material
    }
}

impl Object for Ball {
    fn pos(&self) -> Vec3 {
        self.pos
    }
    fn material(&self) -> Material {
        self.material
    }
}
