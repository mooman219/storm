use bounded_spsc_queue::Consumer;
use bounded_spsc_queue::Producer;
use cgmath::*;
use input::*;
use render::color;
use render::geometry::triangle::*;
use render::message::*;
use render::message::producer::*;
use render::vertex::shape::*;
use time::frame_clock::*;

pub fn game_loop(render_producer: Producer<RenderFrame>, _: Consumer<InputFrame>) {
    let mut render_producer = RenderProducer::new(render_producer);
    for x in -16..4 {
        let offset = x as f32;
        render_producer.create_rect(
            Vector2::new(-1f32 + offset, 0f32),
            Vector2::new(0.5f32, 0.5f32),
            color::ORANGE,
        );
        render_producer.create_rect(
            Vector2::new(-0.5f32 + offset, 0.5f32),
            Vector2::new(0.5f32, 0.5f32),
            color::RED,
        );
        render_producer.create_rect(
            Vector2::new(0f32 + offset, 1f32),
            Vector2::new(0.5f32, 0.5f32),
            color::PURPLE,
        );
        render_producer.create_rect(
            Vector2::new(0.5f32 + offset, 1.5f32),
            Vector2::new(0.5f32, 0.5f32),
            color::BLUE,
        );
    }
    render_producer.create_triangle(Triangle::new(
        ShapeVertex::new(0.0, 1.0, color::RED),
        ShapeVertex::new(-1.0, 0.5, color::BLUE),
        ShapeVertex::new(1.0, 0.5, color::YELLOW),
    ));
    render_producer.send();
    render_producer.create_triangle(Triangle::new(
        ShapeVertex::new(0.0, 1.0, color::RED),
        ShapeVertex::new(1.0, 1.5, color::BLUE),
        ShapeVertex::new(-1.0, 1.5, color::YELLOW),
    ));
    let mut translation = Vector3::new(0f32, 0f32, 0f32);
    let mut clock = FrameClock::new();
    clock.set_fps(60);
    loop {
        if translation.x > 6f32 {
            translation.x = 0f32;
        }
        translation.x += 0.01f32;
        render_producer.set_translation(translation);
        render_producer.send();
        clock.tick();
    }
}
