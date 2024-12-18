use super::Window;

#[derive(Clone, Debug)]
pub struct Monitor {
    pub width: i32,
    pub height: i32,
    pub windows: Vec<Window>,
}

impl Monitor {
    pub const fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            windows: vec![],
        }
    }
}
