use anyhow::Result;

#[derive(Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    mod try_from_string {
        use super::*;

        #[test]
        fn test_valid_left() {
            let result = FocusDirection::try_from_string("left").unwrap();
            assert_eq!(result, FocusDirection::Left);
        }

        #[test]
        fn test_valid_right() {
            let result = FocusDirection::try_from_string("right").unwrap();
            assert_eq!(result, FocusDirection::Right);
        }

        #[test]
        fn test_invalid_direction() {
            let result = FocusDirection::try_from_string("up");
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("Invalid focus direction: up"));
        }

        #[test]
        fn test_empty_string() {
            let result = FocusDirection::try_from_string("");
            assert!(result.is_err());
        }

        #[test]
        fn test_case_sensitive() {
            let result = FocusDirection::try_from_string("Left");
            assert!(result.is_err());
        }
    }

    mod to_int {
        use super::*;

        #[test]
        fn test_left_to_int() {
            assert_eq!(FocusDirection::Left.to_int(), -1);
        }

        #[test]
        fn test_right_to_int() {
            assert_eq!(FocusDirection::Right.to_int(), 1);
        }
    }

    mod from_implementations {
        use super::*;

        #[test]
        fn test_from_focus_direction_for_i32() {
            let left: i32 = FocusDirection::Left.into();
            let right: i32 = FocusDirection::Right.into();

            assert_eq!(left, -1);
            assert_eq!(right, 1);
        }

        #[test]
        fn test_from_focus_direction_ref_for_i32() {
            let left: i32 = (&FocusDirection::Left).into();
            let right: i32 = (&FocusDirection::Right).into();

            assert_eq!(left, -1);
            assert_eq!(right, 1);
        }
    }

    mod try_from_implementations {
        use super::*;

        #[test]
        fn test_try_from_string() {
            let left = FocusDirection::try_from("left".to_string()).unwrap();
            let right = FocusDirection::try_from("right".to_string()).unwrap();

            assert_eq!(left, FocusDirection::Left);
            assert_eq!(right, FocusDirection::Right);

            let invalid = FocusDirection::try_from("invalid".to_string());
            assert!(invalid.is_err());
        }

        #[test]
        fn test_try_from_str() {
            let left = FocusDirection::try_from("left").unwrap();
            let right = FocusDirection::try_from("right").unwrap();

            assert_eq!(left, FocusDirection::Left);
            assert_eq!(right, FocusDirection::Right);

            let invalid = FocusDirection::try_from("invalid");
            assert!(invalid.is_err());
        }
    }
}
