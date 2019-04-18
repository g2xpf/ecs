bitflags! {
    pub struct Component: usize {
        const NONE       = 0b00000000;
        const RENDERABLE = 0b00000001;
        const COLLIDABLE = 0b00000010;
    }
}
