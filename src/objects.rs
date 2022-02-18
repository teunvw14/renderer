use crate::vector::Vec3;

use std::fs::File;
use std::io::{BufReader, BufWriter, Read};
use std::ops::{Deref, DerefMut};
use std::path::Path;

use serde::{Deserialize, Serialize};

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

pub type TriangleFaceIndices = (usize, usize, usize);
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
    pub faces: Vec<TriangleFaceIndices>,
    pub material: Material,
}

impl VertexObject {
    /// Get an iterator over the object's faces. Vertices are given relative to
    /// the object's position.
    pub fn iter_faces(&self) -> FacesIterator {
        FacesIterator::from_vertex_object(self)
    }
}

/// An iterator type used to iterate over the faces of a VertexObject.
pub struct FacesIterator<'a> {
    index: usize,
    vertices: &'a Vec<Vec3>,
    face_indices: &'a Vec<TriangleFaceIndices>,
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
    fn set_pos(&mut self, pos: Vec3);
    fn material(&self) -> Material;
}

impl Object for VertexObject {
    fn pos(&self) -> Vec3 {
        self.pos
    }
    fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }
    fn material(&self) -> Material {
        self.material
    }
}

impl Object for Ball {
    fn pos(&self) -> Vec3 {
        self.pos
    }
    fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }
    fn material(&self) -> Material {
        self.material
    }
}
