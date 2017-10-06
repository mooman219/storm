#![feature(asm, const_fn)]

extern crate cgmath;
extern crate gl;
extern crate glutin;

mod render;

use render::shape::*;
use render::shape::quad::*;
use render::shape::triangle::*;
use render::vertex::pos2_color::*;
use render::display;
use render::frame_clock::*;
use render::shader;

const VS_SRC: &'static [u8] = b"
#version 330

layout(location = 0) in vec2 a_pos;
layout(location = 1) in vec4 a_color;
out vec4 v_color;

void main() {
    gl_Position = vec4(a_pos, 0.0, 1.0);
    v_color = a_color;
}\0";

const FS_SRC: &'static [u8] = b"
#version 330

in vec4 v_color;
out vec4 output;

void main() {
    output = v_color;
}\0";

fn main() {
    // Event loop creation
    let mut events_loop = glutin::EventsLoop::new();
    // Window configuration
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_dimensions(1024, 768);
    let context = glutin::ContextBuilder::new();
    // Winow creation
    let mut display = display::Display::new(window, context, &events_loop);

    println!("OpenGL version {}", display.get_version_string());

    let program = shader::ShaderProgram::new(VS_SRC, FS_SRC);
    program.bind();

    let mut triangle_buffer = Triangle::<Pos2ColorVertex>::new_shape_buffer();
    triangle_buffer.add(Triangle::new(
        Pos2ColorVertex::new(0.0, 0.0, 0.0, 1.0, 0.0, 1.0),
        Pos2ColorVertex::new(-1.0, -1.0, 1.0, 0.0, 0.0, 1.0),
        Pos2ColorVertex::new(1.0, -1.0, 0.0, 0.0, 1.0, 1.0),
    ));
    triangle_buffer.add(Triangle::new(
        Pos2ColorVertex::new(0.0, 0.0, 0.0, 1.0, 0.0, 1.0),
        Pos2ColorVertex::new(1.0, 1.0, 1.0, 0.0, 0.0, 1.0),
        Pos2ColorVertex::new(-1.0, 1.0, 0.0, 0.0, 1.0, 1.0),
    ));
    let mut quad_buffer = Quad::<Pos2ColorVertex>::new_shape_buffer();
    quad_buffer.add(Quad::new(
        Pos2ColorVertex::new(-0.5, 0.5, 1.0, 0.0, 0.0, 1.0),
        Pos2ColorVertex::new(-0.5, -0.5, 0.0, 0.0, 1.0, 1.0),
        Pos2ColorVertex::new(0.5, 0.5, 0.0, 1.0, 0.0, 1.0),
        Pos2ColorVertex::new(0.5, -0.5, 1.0, 1.0, 1.0, 1.0),
    ));

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
