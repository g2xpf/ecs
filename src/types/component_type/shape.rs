use glsl_linalg::float;

#[derive(Debug)]
pub enum Shape<F>
where
    F: float::Float,
{
    Circle { r: F },
    Rectangle { w: F, h: F },
}
