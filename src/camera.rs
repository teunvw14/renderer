use crate::util::Resolution;
use crate::vector::Vec3;

pub struct ImagePlane {
    pub top_left: Vec3,
    pub top_right: Vec3,
    pub bottom_right: Vec3,
    pub bottom_left: Vec3,
}

pub struct Camera {
    pub pos: Vec3,
    pub res: Resolution, // A 2-vector representing the camera resolution.
    pub image_plane: ImagePlane,
}

impl Camera {
    pub fn translate(&mut self, by: Vec3) {
        self.pos += by;
        self.image_plane.top_left += by;
        self.image_plane.top_right += by;
        self.image_plane.bottom_right += by;
        self.image_plane.bottom_left += by;
    }

    pub fn look_at(&mut self, at: Vec3) {
        // Get the direction vector and normalize
        let mut direction = at - self.pos;
        direction.normalize();
        
        // Change the ImagePlane components
        unimplemented!();
        // self.image_plane.top_left = self.pos + ;
        // self.image_plane.top_right = self.pos + ;
        // self.image_plane.bottom_right = self.pos + ;
        // self.image_plane.bottom_left = self.pos + ;
    }
}
