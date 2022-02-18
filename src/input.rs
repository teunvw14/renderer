use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::ControlFlow;
use winit_input_helper::WinitInputHelper;

use pixels::Pixels;

use crate::camera::Camera;

use crate::renderer::MultithreadingMethod;
use crate::renderer::Renderer;
use crate::util::move_pyramid;
use crate::vector::*;
use crate::world::World;

const STEPSIZE: f32 = 0.2;

/// Handle input.
pub fn handle_input(
    input: &WinitInputHelper,
    // input_manager: &mut InputManager,
    control_flow: &mut ControlFlow,
    world: &mut World,
    camera: &mut Camera,
    renderer: &mut Renderer,
    pixels: &mut Pixels,
    multithreading: &mut bool,
    click_count: &mut u8,
) {
    // Check if the left mouse button was pressed.
    if input.mouse_pressed(0) {
        *click_count += 1;
        *click_count = *click_count % 3;
        if let Some(ball) = world.balls.get(*click_count as usize) {
            camera.look_at(ball.pos);
            let c = *click_count + 1;
            println!("Looking at ball: {c:?}");
        }
    }

    // Change the camera FOV:
    if input.key_pressed(VirtualKeyCode::Minus) {
        if camera.get_field_of_view_horizontal_deg() > 1.0 {
            // Unwrap is safe because decreasing the FOV will only return an
            // error if the value is smaller than, or equal to 0.
            camera
                .set_field_of_view_horizontal_deg(camera.get_field_of_view_horizontal_deg() - 1.0)
                .unwrap();
        }
    }
    if input.key_pressed(VirtualKeyCode::Equals) {
        if camera.get_field_of_view_horizontal_deg() < 179.0 {
            // Unwrap is safe because increasing the FOV will only return an
            // error if the value is greater than 180.
            camera
                .set_field_of_view_horizontal_deg(camera.get_field_of_view_horizontal_deg() + 1.0)
                .unwrap();
        }
    }

    // Close events
    if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
        *control_flow = ControlFlow::Exit;
        return;
    }

    // Move camera (for debug purposes).
    if input.key_pressed(VirtualKeyCode::Left) {
        move_pyramid(world, vec3(-STEPSIZE, 0.0, 0.0));
        // camera.translate(vec3(-STEPSIZE, 0.0, 0.0));
    }
    if input.key_pressed(VirtualKeyCode::Right) {
        move_pyramid(world, vec3(STEPSIZE, 0.0, 0.0));
        // camera.translate(vec3(STEPSIZE, 0.0, 0.0));
    }
    if input.key_pressed(VirtualKeyCode::Down) {
        move_pyramid(world, vec3(0.0, -STEPSIZE, 0.0));
        // camera.translate(vec3(0.0, -STEPSIZE, 0.0));
    }
    if input.key_pressed(VirtualKeyCode::Up) {
        move_pyramid(world, vec3(0.0, STEPSIZE, 0.0));
        // camera.translate(vec3(0.0, STEPSIZE, 0.0));
    }
    if input.key_pressed(VirtualKeyCode::End) {
        move_pyramid(world, vec3(0.0, 0.0, -STEPSIZE));
        // camera.translate(vec3(0.0, 0.0, -STEPSIZE));
    }
    if input.key_pressed(VirtualKeyCode::Home) {
        move_pyramid(world, vec3(0.0, 0.0, STEPSIZE));
        // camera.translate(vec3(0.0, 0.0, STEPSIZE));
    }

    if input.key_pressed(VirtualKeyCode::G) {
        renderer.grayscale = !renderer.grayscale;
    }

    if input.key_pressed(VirtualKeyCode::M) {
        renderer.multithreading_method = match renderer.multithreading_method {
            MultithreadingMethod::None => {
                println!("Switching to crossbeam multithreading.");
                MultithreadingMethod::Crossbeam
            }
            MultithreadingMethod::Crossbeam => {
                println!("Switching to rayon multithreading.");
                MultithreadingMethod::Rayon
            }
            MultithreadingMethod::Rayon => {
                println!("Disabled multithreading.");
                MultithreadingMethod::None
            }
        };
    }

    // Resize the window
    if let Some(size) = input.window_resized() {
        pixels.resize_surface(size.width, size.height);
    }
}

// /// InputManager keeps track of all single keypress actions for easy matching.
// pub struct InputManager {
//     pub inputs_of_interest: Vec<VirtualKeyCode>,
//     pub inputs_registered: Vec<VirtualKeyCode>,
// }

// pub fn input_init() -> InputManager {
//     let inputs_of_interest = vec![
//         VirtualKeyCode::Escape,
//         VirtualKeyCode::Left
//     ];
//     let inputs_registered = Vec::with_capacity(inputs_of_interest.len());
//     InputManager { inputs_of_interest, inputs_registered }
// }

// impl InputManager {
//     /// Update which keys of interest were pressed.
//     fn update(&mut self, input: &WinitInputHelper) {
//         self.inputs_registered.clear();
//         for key in &self.inputs_of_interest {
//             if input.key_pressed(*key) {
//                 self.inputs_registered.push(*key);
//             }
//         }
//     }
// }
