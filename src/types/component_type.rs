use glsl_linalg::{float, V2};

pub enum Shape<F> {
    Circle { r: F },
    Rectangle { w: F, h: F },
}

pub struct Rigid<F>
where
    F: float::Float,
{
    pub r: V2<F>,
    pub v: V2<F>,
    pub a: V2<F>,
    pub mass: F,
    pub inv_mass: F,
    pub inertia: F,
    pub inv_inertia: F,
    pub omega: F,
    pub angle: F,

    pub is_static: bool,
}
