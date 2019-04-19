bitflags! {
    pub struct Component: usize {
        const NONE          = 0b00000000;
        const RIGID         = 0b00000001;
        const SHAPE         = 0b00000010;
        const RENDERER      = 0b00000100;
        const EVENT_HANDLER = 0b00001000;
    }
}

impl Component {
    fn contains(self, rhs: Component) -> bool {
        (self & rhs) == rhs
    }
}
