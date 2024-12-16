use super::Window;

#[derive(Clone, Debug)]
pub struct Monitor {
    pub width: usize,
    pub height: usize,
    pub windows: Vec<Window>,
}

impl Monitor {
    pub const fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            windows: vec![],
        }
    }
}
