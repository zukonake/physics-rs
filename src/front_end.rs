use glium;

use map;

#[derive(Copy, Clone, Default)]
struct Vertex
{
    position: [f32; 2],
    color: [f32; 4],
}

implement_vertex!(Vertex, position, color);

pub struct MapRenderer
{
    map_vertices: glium::VertexBuffer<Vertex>,
    map_indices: glium::index::IndexBuffer<u16>,
    entities_vertices: glium::VertexBuffer<Vertex>,
    entities_indices: glium::index::NoIndices,
}

impl MapRenderer
{
    pub fn new(display: &glium::backend::glutin::Display) -> Self
    {//TODO errors
        const QUADS_NUMBER: usize = map::WIDTH * map::HEIGHT * 4;
        let mut vertices: [Vertex; QUADS_NUMBER] = [Vertex::default(); QUADS_NUMBER];
        for i in 0..(map::WIDTH * map::HEIGHT)
        {
            let left = (i % map::WIDTH) as f32;
            let top = (i / map::WIDTH) as f32;
            vertices[(i * 4) + 0].position = [top, left];
            vertices[(i * 4) + 1].position = [top, left + 1.0];
            vertices[(i * 4) + 2].position = [top + 1.0, left + 1.0];
            vertices[(i * 4) + 3].position = [top + 1.0, left];
        }
        let map_vertices = glium::VertexBuffer::new(display, &vertices).unwrap();
        let map_indices =
            glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList,
                                           &[0, 1, 2, 0, 2, 3][..]).unwrap();
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
}
