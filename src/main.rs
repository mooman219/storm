#![feature(asm, const_fn, const_size_of)]

extern crate cgmath;
extern crate gl;
extern crate glutin;

mod render;
mod time;

use render::shape::*;
use render::shape::quad::*;
use render::shape::triangle::*;
use render::vertex::pos2::*;
use render::display;
use render::shader::*;
use time::frame_clock::*;

fn main() {
    // Event loop creation
    let mut events_loop = glutin::EventsLoop::new();
    // Window configuration
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_dimensions(1024, 768);
    let context = glutin::ContextBuilder::new().with_vsync(true);
    // Winow creation
    let mut display = display::Display::new(window, context, &events_loop);

    println!("OpenGL version {}", display.get_version_string());

    // Initialize the shaders
    let program = program::ShaderProgram::new(
        include_str!["render/shader/shape.glslv"],
        include_str!["render/shader/shape.glslf"],
    );
    program.bind();

    // Setup shape buffers
    let mut triangle_buffer = Triangle::new_shape_buffer();
    triangle_buffer.add(Triangle::new(
        Pos2Vertex::new(0.0, 0.0, 0.0, 1.0, 0.0, 1.0),
        Pos2Vertex::new(-1.0, -1.0, 1.0, 0.0, 0.0, 1.0),
        Pos2Vertex::new(1.0, -1.0, 0.0, 0.0, 1.0, 1.0),
    ));
    triangle_buffer.add(Triangle::new(
        Pos2Vertex::new(0.0, 0.0, 0.0, 1.0, 0.0, 1.0),
        Pos2Vertex::new(1.0, 1.0, 1.0, 0.0, 0.0, 1.0),
        Pos2Vertex::new(-1.0, 1.0, 0.0, 0.0, 1.0, 1.0),
    ));
    let mut quad_buffer = Quad::new_shape_buffer();
    quad_buffer.add(Quad::new(
        Pos2Vertex::new(-0.5, 0.5, 1.0, 0.0, 0.0, 1.0),
        Pos2Vertex::new(-0.5, -0.5, 0.0, 0.0, 1.0, 1.0),
        Pos2Vertex::new(0.5, 0.5, 0.0, 1.0, 0.0, 1.0),
        Pos2Vertex::new(0.5, -0.5, 1.0, 1.0, 1.0, 1.0),
    ));

    // Sync with the gpu
    triangle_buffer.sync();
    quad_buffer.sync();

    let mut clock = FrameClock::new();

    display.enable_clear_color();
    display.clear_color(0.0, 0.0, 0.0, 1.0);

    let mut running = true;
    while running {
        // Input
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => running = false,
                glutin::WindowEvent::Resized(w, h) => display.resize(w, h),
                _ => (),
            },
            _ => (),
        });

        // Render
        triangle_buffer.draw();
        quad_buffer.draw();
        display.swap_buffers();
        display.clear();

        // Frames
        clock.tick();
    }
}
