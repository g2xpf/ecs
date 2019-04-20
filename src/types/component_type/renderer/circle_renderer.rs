use glium::{index, vertex, Display, DrawParameters, Program};

const SPLIT_COUNT: u32 = 100;
static CIRCLE_VS_SOURCE: &'static str = include_str!("circle.vert");
static CIRCLE_FS_SOURCE: &'static str = include_str!("circle.frag");

#[derive(Copy, Clone)]
pub struct Vertex {
    pub coord: [f32; 2],
}
implement_vertex!(Vertex, coord);

pub struct CircleRenderer<'a> {
    pub program: Program,
    pub index_buffer: index::IndexBuffer<u32>,
    pub vertex_buffer: vertex::VertexBuffer<Vertex>,
    pub draw_parameter: DrawParameters<'a>,
}

impl<'a> CircleRenderer<'a> {
    pub fn new(display: &Display, draw_parameter: DrawParameters<'a>) -> Self {
        CircleRenderer {
            program: glium::Program::from_source(display, CIRCLE_VS_SOURCE, CIRCLE_FS_SOURCE, None)
                .unwrap(),
            vertex_buffer: {
                let mut v = Vec::new();
                for i in 0..SPLIT_COUNT {
                    let theta = (i as f32) * 2.0 * std::f32::consts::PI / (SPLIT_COUNT as f32);
                    v.push(Vertex {
                        coord: [theta.cos(), theta.sin()],
                    })
                }
                v.push(Vertex { coord: [0.0, 0.0] });
                glium::VertexBuffer::new(display, &v[..]).unwrap()
            },
            index_buffer: {
                let mut v = Vec::new();
                for i in 0..SPLIT_COUNT {
                    v.push(i);
                    v.push((i + 1) % SPLIT_COUNT);
                    v.push(SPLIT_COUNT);
                }
                glium::IndexBuffer::new(display, index::PrimitiveType::TrianglesList, &v[..])
                    .unwrap()
            },
            draw_parameter: draw_parameter,
        }
    }
}
