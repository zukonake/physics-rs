#![feature(slice_patterns)]

extern crate time;
#[macro_use]
extern crate glium;

mod world;
mod entity;
mod front_end;

use time::PreciseTime;

use glium::glutin::Event;
use glium::glutin::WindowEvent;
use glium::glutin::ElementState;
use glium::glutin::VirtualKeyCode;
use glium::glutin::MouseButton;
use glium::glutin::KeyboardInput;

fn main()
{
    let mut client = front_end::Client::new();
    let mut world_renderer = front_end::WorldRenderer::new(&client.display);
    let mut world = world::World::new();
    let window_size = client.window_size();
    let scale = [window_size.0 / world::WIDTH as f32, window_size.1 / world::HEIGHT as f32];

    let matrix: [[f32; 3]; 3] =
       [[scale[0], 0.0, 0.0],
        [0.0, scale[1], 0.0],
        [0.0, 0.0, 1.0]];

    let uniforms = uniform!
    {
        matrix: matrix,
        screen_size: window_size,
    };

    let mut mouse_position: [isize; 2] = [0, 0];
    #[derive(PartialEq)]
    enum RunState
    {
        Running,
        Paused,
        Skipping,
        Exited,
    }
    #[derive(PartialEq)]
    enum Action
    {
        None,
        EntitiesBrush,
        WallsBrush,
        DrainsBrush,
        PositivePressureBrush,
        NegativePressureBrush,
    }

    let mut simulation_state = RunState::Running;
    let mut action = Action::None;

    while simulation_state != RunState::Exited
    {
        if simulation_state != RunState::Paused
        {
            let start = PreciseTime::now();
            world.simulate();
            let end = PreciseTime::now();
            println!("\tsimulation: {}us", start.to(end).num_microseconds().unwrap() as f32);
            if simulation_state == RunState::Skipping
            {
                simulation_state = RunState::Paused;
            }
        }
        world_renderer.update(&world, &client.display);
        client.clear_color((0.0, 0.0, 0.0, 1.0));
        client.draw(&world_renderer.map_vertices, &world_renderer.map_indices, &uniforms);
        client.draw(&world_renderer.entities_vertices, &world_renderer.entities_indices, &uniforms);
        client.display();
        client.events_loop.poll_events(|event: Event|
        {
            match event
            {
                Event::WindowEvent{event, ..} => match event
                {
                    WindowEvent::Closed => simulation_state = RunState::Exited,
                    WindowEvent::MouseMoved{position, ..} =>
                        mouse_position = [((position.0 + 0.5) / scale[0] as f64) as world::Coordinate,
                                          ((position.1 + 0.5) / scale[1] as f64) as world::Coordinate],
                    WindowEvent::KeyboardInput{input: KeyboardInput{virtual_keycode, state, ..}, ..} =>
                        match virtual_keycode
                        {
                            Some(VirtualKeyCode::E) =>
                                if state == ElementState::Pressed
                                { action = Action::EntitiesBrush;
                                }
                                else if action == Action::EntitiesBrush &&
                                        state == ElementState::Released
                                {
                                    action = Action::None;
                                },
                            Some(VirtualKeyCode::R) =>
                                if state == ElementState::Pressed
                                {
                                    action = Action::DrainsBrush;
                                }
                                else if action == Action::DrainsBrush &&
                                        state == ElementState::Released
                                {
                                    action = Action::None;
                                },
                            Some(VirtualKeyCode::Q) =>
                                if state == ElementState::Pressed
                                {
                                    if simulation_state == RunState::Paused
                                    {
                                        simulation_state = RunState::Running;
                                    }
                                    else
                                    {
                                        simulation_state = RunState::Paused;
                                    }
                                },
                            Some(VirtualKeyCode::Comma) =>
                                if state == ElementState::Pressed
                                {
                                    if simulation_state == RunState::Paused
                                    {
                                        simulation_state = RunState::Skipping;
                                    }
                                },
                            Some(VirtualKeyCode::Escape) =>
                                simulation_state = RunState::Exited,
                            _ => (),
                        },
                    WindowEvent::MouseInput{button, state, ..} =>
                        match button
                        {
                            MouseButton::Left =>
                                if state == ElementState::Pressed
                                {
                                    action = Action::PositivePressureBrush;
                                }
                                else if action == Action::PositivePressureBrush &&
                                        state == ElementState::Released
                                {
                                    action = Action::None;
                                },
                            MouseButton::Right =>
                                if state == ElementState::Pressed
                                {
                                    action = Action::NegativePressureBrush;
                                }
                                else if action == Action::NegativePressureBrush &&
                                        state == ElementState::Released
                                {
                                    action = Action::None;
                                },
                            MouseButton::Middle =>
                                if state == ElementState::Pressed
                                {
                                    action = Action::WallsBrush;
                                }
                                else if action == Action::WallsBrush &&
                                        state == ElementState::Released
                                {
                                    action = Action::None;
                                },
                            _ => (),
                        },
                        _ => (),
                }
                _ => (),
            }
        });
        match action
        {
            Action::EntitiesBrush => world.place_entity(mouse_position),
            Action::WallsBrush => world.brush(world::Tile::Wall, mouse_position, 1.0),
            Action::DrainsBrush => world.brush(world::Tile::Drain, mouse_position, 1.0),
            Action::PositivePressureBrush => world.brush(world::Tile::Empty(1.0), mouse_position, 3.0),
            Action::NegativePressureBrush => world.brush(world::Tile::Empty(-1.0), mouse_position, 3.0),
            _ => (),
        }
    }
}
