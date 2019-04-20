use crate::types::component_type;
use glium;
use glium::Surface;

pub mod circle_renderer;
pub mod rectangle_renderer;

pub struct Renderer<'a> {
    pub display: glium::Display,
    pub target: Option<glium::Frame>,
    pub circle_renderer: circle_renderer::CircleRenderer<'a>,
    pub rectangle_renderer: rectangle_renderer::RectangleRenderer<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(display: glium::Display) -> Renderer<'a> {
        let circle_renderer = circle_renderer::CircleRenderer::new(&display, Default::default());
        let rectangle_renderer =
            rectangle_renderer::RectangleRenderer::new(&display, Default::default());
        Renderer {
            display,
            target: None,
            circle_renderer,
            rectangle_renderer,
        }
    }

    pub fn draw(&mut self, rigid: &component_type::Rigid, shape: &component_type::Shape) {
        let target = self.target.as_mut().expect("Target not created");
        match *shape {
            component_type::Shape::Circle { r } => {
                target
                    .draw(
                        &self.circle_renderer.vertex_buffer,
                        &self.circle_renderer.index_buffer,
                        &self.circle_renderer.program,
                        &uniform! {
                            radius: r,
                            r: Into::<[f32; 2]>::into((*rigid).r),
                            angle: (*rigid).angle,
                        },
                        &self.circle_renderer.draw_parameter,
                    )
                    .unwrap();
            }
            component_type::Shape::Rectangle { w, h } => {
                target
                    .draw(
                        &self.rectangle_renderer.vertex_buffer,
                        &self.rectangle_renderer.index_buffer,
                        &self.rectangle_renderer.program,
                        &uniform! {
                            w: w,
                            h: h,
                            r: Into::<[f32; 2]>::into((*rigid).r),
                            angle: (*rigid).angle,
                        },
                        &self.rectangle_renderer.draw_parameter,
                    )
                    .unwrap();
            }
        }
    }
}
