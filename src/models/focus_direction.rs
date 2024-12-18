use anyhow::Result;

#[derive(Clone, Debug)]
pub enum FocusDirection {
    Left,
    Right,
}

impl FocusDirection {
    pub fn try_from_string(value: &str) -> Result<Self> {
        if value == "left" {
            Ok(FocusDirection::Left)
        } else if value == "right" {
            Ok(FocusDirection::Right)
        } else {
            Err(anyhow::anyhow!("Invalid focus direction: {}", value))
        }
    }

    pub fn to_int(&self) -> i32 {
        match self {
            FocusDirection::Left => -1,
            FocusDirection::Right => 1,
        }
    }
}

impl From<FocusDirection> for i32 {
    fn from(value: FocusDirection) -> Self {
        value.to_int()
    }
}

impl From<&FocusDirection> for i32 {
    fn from(value: &FocusDirection) -> Self {
        value.to_int()
    }
}

impl TryFrom<String> for FocusDirection {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        FocusDirection::try_from_string(&value)
    }
}

impl TryFrom<&str> for FocusDirection {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        FocusDirection::try_from_string(value)
    }
}
