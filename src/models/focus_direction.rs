use anyhow::Result;

#[derive(Clone, Debug)]
pub enum FocusDirection {
    Left,
    Right,
}

impl TryFrom<i32> for FocusDirection {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self> {
        match value {
            -1 => Ok(FocusDirection::Left),
            1 => Ok(FocusDirection::Right),
            _ => Err(anyhow::anyhow!("Invalid focus direction: {}", value)),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<i32> for FocusDirection {
    fn into(self) -> i32 {
        match self {
            FocusDirection::Left => -1,
            FocusDirection::Right => 1,
        }
    }
}

impl TryFrom<String> for FocusDirection {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        if value == "left" {
            Ok(FocusDirection::Left)
        } else if value == "right" {
            Ok(FocusDirection::Right)
        } else {
            Err(anyhow::anyhow!("Invalid focus direction: {}", value))
        }
    }
}
