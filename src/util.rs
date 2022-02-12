use crate::vector::Vec3;
use crate::world::World;

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