use std;
use std::io::Read;

use glium;
use glium::glutin;
use glium::Surface;

use world;
use entity;

type Point = [f32; 2];
type Size = [f32; 2];
type Color = [f32; 4];
type Index = u32;

#[derive(Copy, Clone, Default)]
pub struct Vertex
{
    position: Point,
    color: Color,
}

implement_vertex!(Vertex, position, color);

pub struct Client
{
    pub events_loop: glutin::EventsLoop,
    pub display: glium::backend::glutin::Display,
    program: glium::Program,
    current_frame: glium::Frame,
}

impl Client
{
    pub fn new() -> Self
    {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title("boids-rs")
            .with_decorations(true);
        let context = glutin::ContextBuilder::new();
        let display = glium::backend::glutin::Display::new(window, context, &events_loop).unwrap();

        {
            let window = &display.gl_window();
            let monitor_id = window.get_current_monitor();
            window.set_fullscreen(Some(monitor_id));
        }

        let mut shader_file = std::fs::File::open("glsl/vertex.vert")
            .expect("Vertex shader file not found.");
        let mut vertex_shader = String::new();
        shader_file.read_to_string(&mut vertex_shader)
            .expect("Couldn't load vertex shader.");

        let mut shader_file = std::fs::File::open("glsl/fragment.frag")
            .expect("Fragment shader file not found.");
        let mut fragment_shader = String::new();
        shader_file.read_to_string(&mut fragment_shader)
            .expect("Couldn't load fragment shader.");

        let program = glium::Program::from_source(&display, &vertex_shader, &fragment_shader, None).unwrap();
        let current_frame = display.draw();

        Self
        {
            events_loop: events_loop,
            display: display,
            program: program,
            current_frame: current_frame,
        }
    }

    pub fn draw<'a, I, U> (&mut self,
                       vertices: &glium::VertexBuffer<Vertex>,
                       indices: I,
                       uniforms: &U) -> ()
    where
        I: Into<glium::index::IndicesSource<'a>>,
        U: glium::uniforms::Uniforms,
    {
        self.current_frame.draw(vertices, indices, &self.program, uniforms,
                                &Default::default()).unwrap()
    }

    pub fn clear_color(&mut self, [r, g, b, a]: Color) -> ()
    {
        self.current_frame.clear_color(r, g, b, a)
    }

    pub fn display(&mut self) -> ()
    {
        self.current_frame.set_finish().unwrap();
        self.current_frame = self.display.draw();
    }

    pub fn window_size(&self) -> Size
    {
        let (w, h) = self.display.gl_window().get_inner_size().unwrap();
        [w as f32, h as f32]
    }
}

impl Drop for Client
{
    fn drop(&mut self) -> ()
    {
        self.current_frame.set_finish().unwrap();
    }
}

const ENTITY_SIZE: f32 = 2.0;

pub struct WorldRenderer
{
    pub map_vertices: glium::VertexBuffer<Vertex>,
    pub map_indices: glium::index::IndexBuffer<Index>,
    pub entities_vertices: glium::VertexBuffer<Vertex>,
    pub entities_indices: glium::index::NoIndices,
}

impl WorldRenderer
{
    pub fn new(display: &glium::backend::glutin::Display) -> Self
    {//TODO errors
        const QUADS_NUMBER: usize = world::WIDTH * world::HEIGHT * 4;
        let mut vertices: [Vertex; QUADS_NUMBER] = [Vertex::default(); QUADS_NUMBER];
        for i in 0..(world::WIDTH * world::HEIGHT)
        {
            let x = (i % world::WIDTH) as f32;
            let y = (i / world::WIDTH) as f32;
            vertices[(i * 4) + 0].position = [x      , y      ];
            vertices[(i * 4) + 1].position = [x + 1.0, y      ];
            vertices[(i * 4) + 2].position = [x + 1.0, y + 1.0];
            vertices[(i * 4) + 3].position = [x      , y + 1.0];
        }
        let map_vertices = glium::VertexBuffer::dynamic(display, &vertices).unwrap();

        const INDICES_NUMBER: usize = world::WIDTH * world::HEIGHT * 6;
        let mut indices: [Index; INDICES_NUMBER] = [0; INDICES_NUMBER];
        for i in 0..(world::WIDTH * world::HEIGHT)
        {
            indices[(i * 6) + 0] = (i as Index * 4) + 0;
            indices[(i * 6) + 1] = (i as Index * 4) + 1;
            indices[(i * 6) + 2] = (i as Index * 4) + 2;
            indices[(i * 6) + 3] = (i as Index * 4) + 0;
            indices[(i * 6) + 4] = (i as Index * 4) + 2;
            indices[(i * 6) + 5] = (i as Index * 4) + 3;
        }
        let map_indices =
            glium::index::IndexBuffer::dynamic(display, glium::index::PrimitiveType::TrianglesList,
                                           &indices).unwrap();
        let entities_vertices = glium::VertexBuffer::new(display, &[]).unwrap();
        let entities_indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        Self
        {
            map_vertices,
            map_indices,
            entities_vertices,
            entities_indices,
        }
    }

    pub fn update(&mut self, world: &world::World, display: &glium::backend::glutin::Display) -> ()
    {
        let ref mut vertices = self.map_vertices.map();
        for i in 0..(world::WIDTH * world::HEIGHT)
        {
            let x = i % world::WIDTH;
            let y = i / world::WIDTH;
            let position = [x as world::Coordinate, y as world::Coordinate];
            let color = match world.at(position)
            {
                Some(tile) => match tile
                {
                    &world::Tile::Empty(value) =>
                    {
                        if value > 0.0
                        {
                            [value.sqrt().sqrt(), value.sqrt(), value, 1.0]
                        }
                        else
                        {
                            [value.abs().sqrt(), value.abs(), value.abs().sqrt().sqrt(), 1.0]
                        }
                    },
                    &world::Tile::Wall => [0.5, 0.5, 0.5, 1.0],
                    &world::Tile::Drain => [0.5, 0.0, 0.0, 1.0],
                },
                None => [0.0, 0.0, 0.0, 1.0],
            };
            for j in 0..4
            {
                vertices[(i * 4) + j].color = color;
            }
        }
        //TODO check for size change instead of resizing every time
        if world.entities.0.len() * std::mem::size_of::<Vertex>() * 3 != self.entities_vertices.get_size()
        {
            let mut vertices = Vec::new();
            for i in 0..world.entities.0.len()
            {
                let vertices_positions = Self::get_entity_vertices_positions(&world.entities.0[i]);
                let color = [1.0, 1.0, 1.0, 0.5];
                let mut triangle =
                vec!
                [
                    Vertex{position: vertices_positions[0], color: color},
                    Vertex{position: vertices_positions[1], color: color},
                    Vertex{position: vertices_positions[2], color: color},
                ];
                vertices.append(&mut triangle);
            }
            self.entities_vertices = glium::VertexBuffer::dynamic(display, &vertices).unwrap();
        }
        else
        {
            let mut vertices = self.entities_vertices.map();
            for i in 0..world.entities.0.len()
            {
                let vertices_positions = Self::get_entity_vertices_positions(&world.entities.0[i]);
                vertices[(i * 3) + 0].position = vertices_positions[0];
                vertices[(i * 3) + 1].position = vertices_positions[1];
                vertices[(i * 3) + 2].position = vertices_positions[2];
            }
        }
    }

    fn get_entity_vertices_positions(entity: &entity::Entity) -> [Point; 3]
    {
        use entity::Point;
        use entity::Vector;
        let rotate_point = |[cx, cy]: Point, angle: f32, [px, py]: Point| -> Point
        {
            let x = angle.cos() * (px - cx) - angle.sin() * (py - cy) + cx;
            let y = angle.sin() * (px - cx) + angle.cos() * (py - cy) + cy;
            [x, y]
        };
        let normalize = |[x, y]: Vector| -> Vector
        {
            let length = (x.powi(2) + y.powi(2)).sqrt();
            if length == 0.0
            {
                [0.0, 0.0]
            }
            else
            {
                [x / length, y / length]
            }
        };
        let [ex, ey] = entity.position;
        let center = [ex - (ENTITY_SIZE / 2.0), ey];
        let direction = normalize(entity.velocity);
        let angle = direction[1].atan2(direction[0]);
        [
            rotate_point(center, angle, entity.position),
            rotate_point(center, angle, [ex - ENTITY_SIZE, ey - (ENTITY_SIZE / 2.0)]),
            rotate_point(center, angle, [ex - ENTITY_SIZE, ey + (ENTITY_SIZE / 2.0)]),
        ]
    }
}
