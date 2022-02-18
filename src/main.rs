#![forbid(unsafe_code)]

mod camera;
mod errors;
mod input;
mod objects;
mod renderer;
mod util;
mod vector;
mod world;

use std::f32::consts::PI;
use std::time::Instant;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use camera::{Camera, ImagePlane};
use input::handle_input;
use objects::*;
use renderer::{MultithreadingMethod, Renderer};
use rgb::*;
use util::{load_object_from_file_json, print_frame_time, Resolution};
use vector::*;
use world::World;

#[macro_use]
extern crate quick_error;

fn main() -> Result<(), Error> {
    let resolution_w: u32 = 800;
    let resolution_h: u32 = 600;

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    // let mut input_manager = input_init();
    let window = {
        let size = LogicalSize::new(resolution_w, resolution_h);
        WindowBuilder::new()
            .with_title("Renderer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(resolution_w, resolution_h, surface_texture)?
    };

    let mut world = World::new();
    world.color = RGBA8::new(196, 196, 196, 255);

    let floor: VertexObject = load_object_from_file_json("res/objects/floor.json").unwrap();
    world.vertex_objects.push(floor);

    let pyramid: VertexObject = load_object_from_file_json("res/objects/pyramid.json").unwrap();
    world.vertex_objects.push(pyramid);

    // Add three balls to the world:
    let ball1: Ball = load_object_from_file_json("res/objects/ball1.json").unwrap();
    let ball2: Ball = load_object_from_file_json("res/objects/ball2.json").unwrap();
    let ball3: Ball = load_object_from_file_json("res/objects/ball3.json").unwrap();
    world.balls.push(ball1);
    world.balls.push(ball2);
    world.balls.push(ball3);

    let triangle: VertexObject = load_object_from_file_json("res/objects/triangle.json").unwrap();
    world.vertex_objects.push(triangle);

    // Add two lights to the world:
    let light1_color = LightIntensity::new(120.0, 120.0, 120.0);
    let light1_pos = vec3(0.0, 3.0, 0.0);
    // Add two lights to the scene:
    let light1 = Light {
        pos: light1_pos,
        diffuse_intensity: light1_color,
        specular_intensity: light1_color,
    };
    world.lights.push(light1);

    // let light1_ball = Ball { pos: light1_pos, rad: 0.25, is_light: true,
    //     material: Material { ambient_constant: light1_color, diffuse_constant: 300.0, specular_constant: 1.0, shine: 5.0 } };
    // world.items.push(light1_ball);

    // let light2_color = LightIntensity::new(1000.0, 1000.0, 1000.0);
    // let light2 = Light {
    //     pos: vec3(-2.0, 10.0, 5.0),
    //     diffuse_intensity: light2_color,
    //     specular_intensity: light2_color,
    // };
    // world.lights.push(light2);

    let mut camera = Camera::new(
        vec3(0.0, 2.5, 5.0),
        vec3(0.0, 0.0, -1.0),
        90.0,
        Resolution {
            w: resolution_w,
            h: resolution_h,
        },
    )
    .expect("Failed to create camera, likely because of invalid parameters.");
    camera.look_at(ball1.pos);

    let mut renderer: Renderer = Renderer {
        grayscale: false,
        multithreading_method: MultithreadingMethod::Rayon,
    };

    let app_start = Instant::now();
    let mut frame_time_ms = 0.0;
    let mut multithreading = false;
    let mut click_count: u8 = 0;

    event_loop.run(move |event, _, control_flow| {
        let frame_start = Instant::now();

        // Handle input events
        if input.update(&event) {
            handle_input(
                &input,
                control_flow,
                &mut world,
                &mut camera,
                &mut renderer,
                &mut pixels,
                &mut multithreading,
                &mut click_count,
            );
        }

        let time = app_start.elapsed();
        // Update internal stateand request a redraw
        world.update(frame_time_ms, time);
        // if let Some(pyramid) = world.vertex_objects.get_mut(1) {
        //     let look_at = pyramid.pos() + *pyramid.vertices.get(0).unwrap();
        //     // println!("Looking at: {look_at:?}");
        //     camera.look_at(look_at);
        // }

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            renderer.render_world(&world, &camera, pixels.get_frame());
            // world.draw(&camera, pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| println!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        frame_time_ms = frame_start.elapsed().as_micros() as f32 / 1000.0;
        print_frame_time(frame_time_ms);
        window.request_redraw();
    });
}
