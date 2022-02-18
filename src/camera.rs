use std::f32::consts::PI;

use crate::errors::*;
use crate::util::{Resolution, SphericalCoordinates};
use crate::vector::{vec3, Vec3};

#[derive(Debug, Copy, Clone, Default)]
pub struct ImagePlane {
    pub top_left: Vec3,
    pub top_right: Vec3,
    pub bottom_right: Vec3,
    pub bottom_left: Vec3,
}

/// A minimal camera struct that exists in 3D space.
pub struct Camera {
    pub pos: Vec3,
    field_of_view_horizontal: f32,
    view_direction: Vec3,
    pub image_plane: ImagePlane,
    pub resolution: Resolution, // A 2-vector representing the camera resolution.
}

impl Camera {
    /// Create a new camera. Field of view in degrees.
    pub fn new(
        pos: Vec3,
        view_direction: Vec3,
        field_of_view_horizontal: f32,
        resolution: Resolution,
    ) -> Result<Camera, CameraSettingError> {
        // TODO: make this a result, instead of using assert
        // Using a field of view greater or equal to 180 degrees leads to
        // unspecific behavior, as does a value lower or equal to zero.
        if field_of_view_horizontal >= 180.0 || field_of_view_horizontal <= 0.0 {
            return Err(CameraSettingError::InvalidFOV(field_of_view_horizontal));
        }
        // Internally, field_of_view is used as radians, so we convert here.
        let field_of_view = field_of_view_horizontal * (PI / 180.0);
        let view_direction = view_direction.normalized();
        let mut camera = Camera {
            pos,
            field_of_view_horizontal: field_of_view,
            view_direction,
            image_plane: ImagePlane::default(),
            resolution,
        };
        camera.image_plane = camera.get_image_plane();
        Ok(camera)
    }

    fn get_image_plane(&self) -> ImagePlane {
        // Calculate the vectors pointing to the middle of the side edges
        // without accounting for the rotation of self's view_direction.
        let rotation_angle = self.field_of_view_horizontal / 2.0;
        let len = (1.0 / rotation_angle.cos()).abs();
        let mut right = vec3(rotation_angle.cos(), 0.0, rotation_angle.sin()) * len;
        let mut left = vec3(rotation_angle.cos(), 0.0, -rotation_angle.sin()) * len;

        // Rotate the vectors into place. First rotate up/down (pan/pitch), then
        // rotate around the vertical y-axis (yaw).
        // The angle of the view_vector with the xz-plane.
        // The right and left vectors initially point to the right, so we can
        // pitch by rotating around the y-axis.
        let view_angle_y = self.view_direction.y.asin();
        right.rotate_z_rad(view_angle_y);
        left.rotate_z_rad(view_angle_y);
        // The angle of the view_vector with the positive x-axis.
        let view_angle_xz = self.view_direction.z.atan2(self.view_direction.x);
        right.rotate_y_rad(view_angle_xz);
        left.rotate_y_rad(view_angle_xz);

        // Calculate the vector pointing "up" from the normal, i.e. the vector
        // orthogonal to the normal and the vector pointing to the right.
        let mut up = right.cross_product(self.view_direction);
        let size_up = rotation_angle.tan() / self.get_aspect_ratio();
        up.set_length(size_up);
        // Calculate all the corner's (relative) position.
        let top_left_relative = left + up;
        let top_right_relative = right + up;
        let bottom_right_relative = right - up;
        let bottom_left_relative = left - up;

        ImagePlane {
            top_left: self.pos + top_left_relative,
            top_right: self.pos + top_right_relative,
            bottom_right: self.pos + bottom_right_relative,
            bottom_left: self.pos + bottom_left_relative,
        }
    }

    pub fn translate(&mut self, by: Vec3) {
        self.pos += by;
        self.image_plane.top_left += by;
        self.image_plane.top_right += by;
        self.image_plane.bottom_right += by;
        self.image_plane.bottom_left += by;
    }
    /// Make the camera point towards a point in space.
    pub fn look_at(&mut self, at: Vec3) {
        // Get the direction vector and normalize
        let direction = at - self.pos;
        self.set_view_direction(direction);
    }
    /// Make the camera point in a particular direction.
    pub fn set_view_direction(&mut self, direction: Vec3) {
        // Update the image_plane.
        let direction_normal = direction.normalized();
        self.view_direction = direction_normal;
        self.image_plane = self.get_image_plane();
    }
    pub fn get_view_direction(&self) -> Vec3 {
        self.view_direction
    }
    /// Get the camera's field of view in radians.
    pub fn get_field_of_view_horizontal(&self) -> f32 {
        self.field_of_view_horizontal
    }
    /// Get the camera's field of view in degrees.
    pub fn get_field_of_view_horizontal_deg(&self) -> f32 {
        self.field_of_view_horizontal * (180.0 / PI)
    }
    /// Set the camera's field of view in degrees.
    pub fn set_field_of_view_horizontal_deg(
        &mut self,
        field_of_view_horizontal: f32,
    ) -> Result<(), CameraSettingError> {
        // Translate to radians then use that to set the FOV.
        let angle_radians = field_of_view_horizontal * (PI / 180.0);
        self.set_field_of_view_horizontal(angle_radians)
    }
    /// Set the camera's field of view (in radians).
    pub fn set_field_of_view_horizontal(
        &mut self,
        field_of_view_horizontal: f32,
    ) -> Result<(), CameraSettingError> {
        // Make sure we got a valid
        if field_of_view_horizontal >= PI || field_of_view_horizontal <= 0.0 {
            return Err(CameraSettingError::InvalidFOV(field_of_view_horizontal));
        }
        // Set the new FOV but also update the image plane.
        self.field_of_view_horizontal = field_of_view_horizontal;
        self.image_plane = self.get_image_plane();
        Ok(())
    }
    pub fn get_aspect_ratio(&self) -> f32 {
        self.resolution.w as f32 / self.resolution.h as f32
    }
}
