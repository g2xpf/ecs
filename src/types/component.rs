bitflags! {
    pub struct Component: usize {
        const NONE          = 0b00000000;
        const RIGID         = 0b00000001;
        const SHAPE         = 0b00000010;
        const EVENT_HANDLER = 0b00000100;
    }
}
