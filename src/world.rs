use std::f32::consts::PI;
use std::time::Duration;

use crate::objects::*;

use crate::vector::vec3;

use rgb::*;

pub struct World {
    pub objects: Vec<VertexObject>,
    pub balls: Vec<Ball>,
    pub lights: Vec<Light>,
    pub color: RGBA8,
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            balls: Vec::new(),
            lights: Vec::new(),
            color: RGBA8::new(0, 0, 0, 255),
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    pub fn update(&mut self, _last_frame_time: f32, time: Duration) {
        if let Some(pyramid) = self.objects.get_mut(0) {
            if pyramid.vertices.len() > 3 {
                let time_passed_s = time.as_millis() as f32 / 1000.0;
                *pyramid.vertices.get_mut(2).unwrap() = vec3(
                    (time_passed_s + 0.0 * PI).cos(),
                    0.0,
                    (time_passed_s + 0.0 * PI).sin(),
                ) * 3.0;
                *pyramid.vertices.get_mut(1).unwrap() = vec3(
                    (time_passed_s + 0.5 * PI).cos(),
                    0.0,
                    (time_passed_s + 0.5 * PI).sin(),
                ) * 3.0;
                *pyramid.vertices.get_mut(0).unwrap() = vec3(
                    (time_passed_s + 1.0 * PI).cos(),
                    0.0,
                    (time_passed_s + 1.0 * PI).sin(),
                ) * 3.0;
                *pyramid.vertices.get_mut(3).unwrap() = vec3(
                    (time_passed_s + 1.5 * PI).cos(),
                    0.0,
                    (time_passed_s + 1.5 * PI).sin(),
                ) * 3.0;
            }
        }
    }
}
