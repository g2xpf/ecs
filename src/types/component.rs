bitflags! {
    pub struct Component: usize {
        const NONE          = 0b00000000;
        const RIGID         = 0b00000010;
        const SHAPE         = 0b00000100;
        const RENDERER      = 0b00001000;
        const EVENT_HANDLER = 0b00010000;
    }
}
