use cgmath::Vector2;

#[derive(Debug)]
pub struct Rigid {
    pub r: Vector2<f32>,
    pub v: Vector2<f32>,
    pub a: Vector2<f32>,
    pub mass: f32,
    pub inv_mass: f32,
    pub inertia: f32,
    pub inv_inertia: f32,
    pub omega: f32,
    pub angle: f32,

    pub is_static: bool,
}

impl Default for Rigid {
    fn default() -> Self {
        Rigid {
            r: Vector2::new(0.0, 0.0),
            v: Vector2::new(0.0, 0.0),
            a: Vector2::new(0.0, 0.0),
            mass: Default::default(),
            inv_mass: Default::default(),
            inertia: Default::default(),
            inv_inertia: Default::default(),
            omega: Default::default(),
            angle: Default::default(),

            is_static: false,
        }
    }
}
