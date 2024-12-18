use anyhow::Result;

pub enum FocusDirection {
    Left,
    Right,
}

impl FocusDirection {
    pub fn parse(direction: &str) -> Result<FocusDirection> {
        match direction {
            "left" => Ok(FocusDirection::Left),
            "right" => Ok(FocusDirection::Right),
            _ => Err(anyhow::anyhow!("Invalid focus direction: {direction}")),
        }
    }
}
