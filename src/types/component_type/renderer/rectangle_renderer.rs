use glium::{index, vertex, DrawParameters, Program};

static RECTANGLE_VS_SOURCE: &'static str = include_str!("rectangle.vert");
static RECTANGLE_FS_SOURCE: &'static str = include_str!("rectangle.frag");

#[derive(Copy, Clone)]
pub struct Vertex {
    pub coord: [f32; 2],
}
implement_vertex!(Vertex, coord);

pub struct RectangleRenderer<'a> {
    pub program: Program,
    pub index_buffer: index::IndexBuffer<u32>,
    pub vertex_buffer: vertex::VertexBuffer<Vertex>,
    pub draw_parameter: DrawParameters<'a>,
}

impl<'a> RectangleRenderer<'a> {
    pub fn new(display: &glium::Display, draw_parameter: glium::DrawParameters<'a>) -> Self {
        RectangleRenderer {
            program: glium::Program::from_source(
                display,
                RECTANGLE_VS_SOURCE,
                RECTANGLE_FS_SOURCE,
                None,
            )
            .unwrap(),
            vertex_buffer: glium::VertexBuffer::new(
                display,
                &[
                    Vertex { coord: [1.0, 1.0] },
                    Vertex { coord: [-1.0, 1.0] },
                    Vertex { coord: [1.0, -1.0] },
                    Vertex {
                        coord: [-1.0, -1.0],
                    },
                ],
            )
            .unwrap(),
            index_buffer: glium::IndexBuffer::new(
                display,
                index::PrimitiveType::TrianglesList,
                &[0, 1, 2, 1, 2, 3],
            )
            .unwrap(),
            draw_parameter: draw_parameter,
        }
    }
}
