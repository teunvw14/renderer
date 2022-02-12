#![forbid(unsafe_code)]

mod camera;
mod input;
mod objects;
mod renderer;
mod util;
mod vector;
mod world;

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
use renderer::{Renderer, MultithreadingMethod};
use rgb::*;
use util::{Resolution, print_frame_time };
use vector::*;
use world::World;


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

    let triangle = VertexObject::from_file_json("res/objects/triangle.json").unwrap();

    let pyramid = VertexObject::from_file_json("res/objects/pyramid.json").unwrap();
    world.objects.push(pyramid);

    // Add three balls to the world:
    let ball1 = Ball::from_file_json("res/objects/ball1.json").unwrap();
    let ball2 = Ball::from_file_json("res/objects/ball2.json").unwrap();
    let ball3 = Ball::from_file_json("res/objects/ball3.json").unwrap();
    world.balls.push(ball1);
    world.balls.push(ball2);
    world.balls.push(ball3);
    

    // Add two lights to the world:
    let light1_color = LightIntensity::new(20.0, 20.0, 20.0);
    let light1_pos = vec3(2.0, 2.0, 1.0);
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

    let light2_color = LightIntensity::new(1000.0, 1000.0, 1000.0);
    let light2 = Light {
        pos: vec3(-2.0, -3.0, 5.0),
        diffuse_intensity: light2_color,
        specular_intensity: light2_color,
    };
    world.lights.push(light2);

    let mut camera = Camera {
        pos: vec3(0.0, 0.0, 1.0),
        res: Resolution {
            w: resolution_w,
            h: resolution_h,
        },
        image_plane: ImagePlane {
            top_left: vec3(-1.0, 0.75, 0.0),
            top_right: vec3(1.0, 0.75, 0.0),
            bottom_right: vec3(1.0, -0.75, 0.0),
            bottom_left: vec3(-1.0, -0.75, 0.0),
        },
    };

    let mut renderer: Renderer = Renderer { grayscale: false, multithreading_method: MultithreadingMethod::Rayon };

    let app_start = Instant::now();
    let mut frame_time_ms = 0.0;
    let mut multithreading = false;
    event_loop.run(move |event, _, control_flow| {
        let frame_start = Instant::now();

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
            );
        }

        frame_time_ms = frame_start.elapsed().as_micros() as f32 / 1000.0;

        let time = app_start.elapsed();
        // Update internal stateand request a redraw
        world.update(frame_time_ms, time);
        window.request_redraw();

        print_frame_time(frame_time_ms);
    });
}
