use std::cmp::{min, max};

use crate::camera::Camera;
use crate::objects::*;
use crate::vector::Vec3;
use crate::World;

use rgb::*;
use num_cpus;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Renderer {
    pub grayscale: bool,
    pub multithreading_method: MultithreadingMethod,
}

#[derive(Debug, Clone, Copy)]
pub enum MultithreadingMethod {
    None,
    Rayon,
    Crossbeam,
}

impl Renderer {
    /// Draw the `World` state to the frame buffer.
    pub fn render_world(&self, world: &World, camera: &Camera, frame_buffer: &mut [u8]) {
        match self.multithreading_method {
            MultithreadingMethod::None  => {
                for (i, pixel) in frame_buffer.chunks_exact_mut(4).enumerate() {
                    self.render_pixel(i, pixel, camera, world);
                }
            },
            MultithreadingMethod::Rayon => {
                frame_buffer.par_chunks_exact_mut(4)
                .enumerate()
                .map(|(i, pixel)| self.render_pixel(i, pixel, camera , world))
                .collect::<()>();
            }
            MultithreadingMethod::Crossbeam => {
                // Multithreaded!
                // Each pixel takes up 4 bytes
                let cpu_count = num_cpus::get();
                let pixel_count = frame_buffer.len() / 4;
                let pixels_per_thread = pixel_count / cpu_count;
                crossbeam::scope(|s | {
                    let mut offset: usize = 0;
                    // The length of the array is 4 times the amount of pixels, so we need
                    // add a factor of 4.
                    for chunk in frame_buffer.chunks_mut(4 * pixels_per_thread) {
                        let chunk_size = chunk.len() / 4;
                        s.spawn(move |_| {
                            for (i, pixel) in chunk.chunks_exact_mut(4).enumerate() {
                                let index_offset = i + offset;
                                self.render_pixel(index_offset, pixel, camera, world);
                            }
                        });
                        offset += chunk_size;
                    }
                }).unwrap();
            },
        }
    }

    fn render_pixel(&self, pixel_index: usize, pixel: &mut [u8], camera: &Camera, world: &World) {
            let pixel_ray_direction = Self::calculate_pixel_ray(camera, pixel_index);

            let closest_ball =
                self.get_nearest_intersecting_ball(&world.balls, camera.pos, pixel_ray_direction);
            let closest_triangle = self.get_nearest_intersecting_triangle(
                &world.objects,
                camera.pos,
                pixel_ray_direction,
            );
            let mut hit = true;
            let mut ball_closer = false;
            match (closest_ball, closest_triangle) {
                (Some((_, pos_hit_ball)), Some((_, _, pos_hit_triangle))) => {
                    hit = true;
                    let distance_ball = (camera.pos - pos_hit_ball).len();
                    let distance_triangle = (camera.pos - pos_hit_triangle).len();
                    ball_closer = distance_ball <= distance_triangle;
                }
                (Some(_), None) => ball_closer = true,
                (None, Some(_)) => ball_closer = false,
                (None, None) => hit = false,
            };

            let mut rgba = if hit {
                if ball_closer {
                    let (ball, pos_hit_ball) = closest_ball.unwrap();
                    let ball_normal = pos_hit_ball - ball.pos;
                    self.get_light_color(
                        &world.lights,
                        &world.balls,
                        ball.material,
                        pos_hit_ball,
                        camera,
                        ball_normal,
                    )
                } else {
                    let (vertex_object, (v0, v1, v2), pos_hit_triangle) = closest_triangle.unwrap();
                    let triangle_normal = get_triangle_normal((v0, v1, v2));
                    self.get_light_color(
                        &world.lights,
                        &world.balls,
                        vertex_object.material,
                        pos_hit_triangle,
                        camera,
                        triangle_normal,
                    )
                }
            } else {
                world.color
            };

            self.apply_filters(&mut rgba);

            pixel.copy_from_slice(rgba.as_slice());
    }

    fn calculate_pixel_ray(camera: &Camera, i: usize) -> Vec3 {
            let alpha = (i % camera.resolution.w as usize) as f32 / camera.resolution.w as f32;
            let beta = (i / camera.resolution.w as usize) as f32 / camera.resolution.h as f32;

            let hi = camera.image_plane.top_left * (1.0 - alpha) +
                camera.image_plane.top_right * alpha;
            let lo = camera.image_plane.bottom_left * (1.0 - alpha)
                + camera.image_plane.bottom_right * alpha;
            let pixel_vec = hi * (1.0 - beta) + lo * beta;

            pixel_vec - camera.pos
    }

    fn apply_filters(&self, rgba: &mut RGBA8) {
        if self.grayscale {
            let avg = rgba.r / 3 + rgba.g / 3 + rgba.b / 3;
            rgba.r = avg;
            rgba.g = avg;
            rgba.b = avg;
        }
    }

    // TODO: add a "t value constraint" argument
    /// Get the triangle face nearest to the origin
    fn get_nearest_intersecting_triangle<'a>(
        &self,
        objects: &'a [VertexObject],
        origin: Vec3,
        direction: Vec3,
    ) -> Option<(&'a VertexObject, TriangleFace, Vec3)> {
        let mut t_min = f32::MAX;
        let mut result = None;
        for object in objects {
            for face in object.iter_faces() {
                let (v0_relative, v1_relative, v2_relative) = face;
                // Get the real coordinates (adjusted for the object position)
                let v0 = v0_relative + object.pos;
                let v1 = v1_relative + object.pos;
                let v2 = v2_relative + object.pos;

                // Get the normal:
                let n = get_triangle_normal((v0, v1, v2));

                // Find intersections:

                // First check if the ray and the plane are not parallel. We do
                // this by calculating the dotproduct of the normal N and the
                // direction vector. If this is (close to) 0, it means that the
                // direction is perpendicular to the normal, and thus parallel
                // to the plane.
                if (n * direction).abs() < 0.001 {
                    continue;
                }

                // Calculate d in the plane equation
                // (in linear form: ax + by + cz + d = 0)
                let d = n * v0 * -1.0;
                let t = -(n * origin + d) / (n * direction);
                // Check if the triangle is behind the camera's ImagePlane
                if t < 1.0 {
                    continue;
                }
                if t < t_min {
                    // Check if the intersection between the ray and the plane is
                    // actually inside the triangle.
                    let p = origin + direction * t;
                    // i is the inward-facing vector
                    let mut i: Vec3;

                    // First edge:

                    let edge0 = v1 - v0;
                    i = n.cross_product(edge0);
                    let v0p = p - v0;
                    if i * v0p < 0.0 {
                        continue;
                    }
                    // Second edge:
                    let edge1 = v2 - v1;
                    i = n.cross_product(edge1);
                    let v1p = p - v1;
                    if i * v1p < 0.0 {
                        continue;
                    }
                    // Third edge:
                    let edge2 = v0 - v2;
                    i = n.cross_product(edge2);
                    let v2p = p - v2;
                    if i * v2p < 0.0 {
                        continue;
                    }
                    // We've found an intersection!
                    t_min = t;
                    result = Some((object, face, p));
                }
            }
        }
        result
    }

    // TODO: add a "t value constraint" argument
    fn get_nearest_intersecting_ball<'a>(
        &self,
        balls: &'a [Ball],
        origin: Vec3,
        direction: Vec3,
    ) -> Option<(&'a Ball, Vec3)> {
        let mut result_ball = None;

        let mut t_min: f32 = f32::MAX;
        for ball in balls {
            let center_adj = origin - ball.pos;

            // Apply the quadratic equation:
            let a: f32 = {
                let dir_len = direction.len();
                dir_len * dir_len
            };
            let b: f32 = center_adj * direction * 2.0;
            let c: f32 = center_adj.len() * center_adj.len() - ball.rad * ball.rad;
            let d: f32 = b * b - 4.0 * a * c;
            match d {
                x if x < 0.0 => {
                    // No intersections, move onto the next ball.
                }
                x if x == 0.0 => {
                    let t = -b / 2.0 * a;
                    // t = 1 is exactly on the image plane, so any values t < 1
                    // are intersections that are in front of the plane instead
                    // of behind it
                    if t < t_min && t >= 1.0 {
                        t_min = t;
                        result_ball = Some(ball);
                    }
                }
                x if x > 0.0 => {
                    let t1 = (-b + d.sqrt()) / (2.0 * a);
                    let t2 = (-b - d.sqrt()) / (2.0 * a);
                    if (t1 < t_min && t1 >= 1.0) || (t2 < t_min && t2 >= 1.0) {
                        // Take the smallest t value.
                        let t = t1.min(t2);
                        if t >= 1.0 {
                            t_min = t;
                            result_ball = Some(ball);
                        }
                    }
                }
                _ => {}
            }
        }
        if let Some(ball) = result_ball {
            let p = origin + direction * t_min;
            Some((ball, p))
        } else {
            None
        }
    }

    fn is_in_shadow(
        &self,
        this_ball: &Ball,
        items: &[Ball],
        origin: Vec3,
        direction: Vec3,
    ) -> bool {
        for ball in items {
            if !std::ptr::eq(ball, this_ball) {
                let center_adj = origin - ball.pos;

                // Apply the quadratic equation:
                let a: f32 = {
                    let dir_len = direction.len();
                    dir_len * dir_len
                };
                let b: f32 = center_adj * direction * 2.0;
                let c: f32 = center_adj.len() * center_adj.len() - ball.rad * ball.rad;
                let d: f32 = b * b - 4.0 * a * c;
                match d {
                    x if x < 0.0 => {
                        // No intersections, move onto the next ball.
                    }
                    x if x == 0.0 => {
                        let t = -b / 2.0 * a;
                        if 0.0 < t && t < 1.0 {
                            return true;
                        }
                    }
                    x if x > 0.0 => {
                        let t1 = (-b + d.sqrt()) / (2.0 * a);
                        let t2 = (-b - d.sqrt()) / (2.0 * a);
                        if (0.0 < t1 && t1 < 1.0) || (0.0 < t2 && t2 < 1.0) {
                            return true;
                        }
                    }
                    _ => {}
                }
            }
        }
        false
    }

    fn get_light_color(
        &self,
        lights: &[Light],
        _balls: &[Ball],
        material: Material,
        pos: Vec3,
        camera: &Camera,
        surface_normal: Vec3,
    ) -> RGBA8 {
        let ambient_r = material.ambient_constant.r as usize; // * self.color.r as usize;
        let ambient_g = material.ambient_constant.g as usize; // * self.color.g as usize;
        let ambient_b = material.ambient_constant.b as usize; // * self.color.b as usize;

        let mut diffuse_r: usize = 0;
        let mut diffuse_g: usize = 0;
        let mut diffuse_b: usize = 0;

        let mut specular_r: usize = 0;
        let mut specular_g: usize = 0;
        let mut specular_b: usize = 0;

        for light in lights {
            // if !self.is_in_shadow(ball, balls, pos, light.pos - pos) {
                let surface_normal = surface_normal.normalized();
                let p_to_light_normal = (light.pos - pos).normalized();
                let dot_product = p_to_light_normal * surface_normal;
                if dot_product >= 0.0 {
                    let distance_to_light = (light.pos - pos).len();
                    let d_sq = distance_to_light * distance_to_light;
                    // Diffuse:
                    diffuse_r += (dot_product
                        * material.diffuse_constant
                        * light.diffuse_intensity.r
                        / d_sq) as usize;
                    diffuse_g += (dot_product
                        * material.diffuse_constant
                        * light.diffuse_intensity.g
                        / d_sq) as usize;
                    diffuse_b += (dot_product
                        * material.diffuse_constant
                        * light.diffuse_intensity.b
                        / d_sq) as usize;

                    // Specular:
                    let reflectance_vector =
                        ((surface_normal * 2.0 * dot_product) - p_to_light_normal).normalized();
                    let view_vector = (camera.pos - pos).normalized();
                    let dot_product_view = reflectance_vector * view_vector;
                    let specular_factor = dot_product_view.powf(material.shine);
                    if dot_product_view >= 0.0 {
                        specular_r += (light.specular_intensity.r
                            * material.specular_constant
                            * specular_factor
                            / d_sq) as usize;
                        specular_g += (light.specular_intensity.g
                            * material.specular_constant
                            * specular_factor
                            / d_sq) as usize;
                        specular_b += (light.specular_intensity.b
                            * material.specular_constant
                            * specular_factor
                            / d_sq) as usize;
                    }
                // }
            }
        }

        let r = ambient_r / 3 + diffuse_r / 3 + specular_r / 3;
        let g = ambient_g / 3 + diffuse_g / 3 + specular_g / 3;
        let b = ambient_b / 3 + diffuse_b / 3 + specular_b / 3;

        let r = min(r, 255) as u8;
        let g = min(g, 255) as u8;
        let b = min(b, 255) as u8;
        RGBA8 { r, g, b, a: 255 }
    }
}
