use std::f32::consts::PI;

use crate::util::{Resolution, SphericalCoordinates};
use crate::vector::{Vec3, vec3};

#[derive(Debug, Copy, Clone, Default)]
pub struct ImagePlane {
    pub top_left: Vec3,
    pub top_right: Vec3,
    pub bottom_right: Vec3,
    pub bottom_left: Vec3,
}

pub struct Camera {
    pub pos: Vec3,
    field_of_view: f32,
    view_direction: Vec3,
    pub image_plane: ImagePlane,
    pub resolution: Resolution, // A 2-vector representing the camera resolution.
}

impl Camera {
    /// Create a new camera. Field of view in degrees.
    pub fn new(pos: Vec3, view_direction: Vec3, field_of_view: f32, resolution: Resolution) -> Camera {
        // TODO: make this a result, instead of using assert
        // Using a field of view greater or equal to 180 degrees breaks everything.
        assert!(field_of_view < 180.0);
        // Internally, field_of_view is used as radians, so we convert here.
        let field_of_view = field_of_view * (PI / 180.0);
        let view_direction = view_direction.normalized();
        let mut camera = Camera { 
            pos,
            field_of_view,
            view_direction,
            image_plane: ImagePlane::default(),
            resolution,
        };
        camera.image_plane = camera.get_image_plane();
        camera
    }

    // TODO: simplify; a lot of steps here are only there to make it easier to
    // understand - but are probably very slow.
    /// Create the image plane based on the camera position, view direction, field of view and aspect ratio.
    fn get_image_plane(&self) -> ImagePlane {
        // First we calculate the directions of the ImagePlane corners, and them
        // translate them by the camera's position.

        panic!("This function is broken! For directions where z != 0, the image plane is somehow skewed. Please investigate!");

        // To rotate around the y-axis, we need to make a conversion (because we
        // can't rotate around the y-axis easily using polar coordinates, only
        // around the x- and z- axes.) We do this by first converting a point
        // (x, y, z) to (z, x, y), then rotating around the z-axis, and then
        // converting back: (x, y, z) to (y, z, x).
        // First conversion:
        let direction_adjusted = vec3(self.view_direction.z, self.view_direction.x, self.view_direction.y);
        // Get the spherical coordinates to rotate:
        let direction_spherical = SphericalCoordinates::from(direction_adjusted);
        // Now we rotate:
        let rotation_angle = self.field_of_view / 2.0;
        let mut right_spherical = direction_spherical;
        let mut left_spherical = direction_spherical;
        // Rotation is counter-clockwise, so we subtract the rotation angle to 
        // rotate "to the right".
        right_spherical.phi -= rotation_angle;
        left_spherical.phi += rotation_angle;
        // Do the conversion back to cartesian coordinates.
        // The length needs to be adjusted to make sure the right_cartesian 
        // left_cartesian are on the ImagePlane.
        let len = (1.0/rotation_angle.cos()).abs();
        let right_rotated = Vec3::from(right_spherical) * len;
        let left_rotated = Vec3::from(left_spherical) * len;
        // Now we do the rotation conversion:
        let right = vec3(right_rotated.y, right_rotated.z, right_rotated.x);
        let left = vec3(left_rotated.y, left_rotated.z, left_rotated.x);
        // Calculate the vector pointing "up" from the normal, i.e. the vector 
        // orthogonal to the normal and the vector pointing to the right.
        let up_normal = right.cross_product(self.view_direction).normalized();
        let aspect_ratio = self.resolution.w as f32 / self.resolution.h as f32;
        let size_up = rotation_angle.tan() / aspect_ratio;
        let up = up_normal * size_up;
        // Calculate all the corner's (relative) position.
        let top_left_relative = left + up;
        let top_right_relative = right + up;
        let bottom_right_relative = right - up;
        let bottom_left_relative = left - up;

        let result = ImagePlane {
            top_left: self.pos + top_left_relative,
            top_right: self.pos + top_right_relative,
            bottom_right: self.pos + bottom_right_relative,
            bottom_left: self.pos + bottom_left_relative,
        };
        println!("ImagePlane is {result:?}");
        result
    }

    pub fn translate(&mut self, by: Vec3) {
        self.pos += by;
        self.image_plane.top_left += by;
        self.image_plane.top_right += by;
        self.image_plane.bottom_right += by;
        self.image_plane.bottom_left += by;
    }

    pub fn look_at(&mut self, at: Vec3) {
        // Get the direction vector and normalize
        let direction = at - self.pos;
        self.set_view_direction(direction);
    }

    pub fn set_view_direction(&mut self, direction: Vec3) {
        // Update the image_plane. 
        let direction_normal = direction.normalized(); 
        self.view_direction = direction_normal;
        self.image_plane = self.get_image_plane();
    }
    pub fn field_of_view_rad(&self) -> f32 {
        self.field_of_view
    }
    pub fn field_of_view_deg(&self) -> f32 {
        self.field_of_view * (180.0 / PI)
    }
    pub fn set_field_of_view_deg(&mut self, field_of_view: f32) {
        // Translate to radians then use that to set the FOV. 
        let field_of_view = field_of_view * (PI / 180.0);
        self.set_field_of_view_rad(field_of_view);
    }
    pub fn set_field_of_view_rad(&mut self, field_of_view: f32) {
        // TODO: do something else than assert
        assert!(field_of_view < PI);
        // Set the new FOV but also update the image plane.
        self.field_of_view = field_of_view;
        self.image_plane = self.get_image_plane();
    }
}
