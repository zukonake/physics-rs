#[macro_use]
extern crate glium;

mod map;
mod front_end;

fn main()
{
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("boids-rs")
        .with_decorations(true);
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::backend::glutin::Display::new(window, context, &events_loop).unwrap();

    let map_renderer = front_end::MapRenderer::new(&display);
}
