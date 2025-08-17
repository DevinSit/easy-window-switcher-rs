use anyhow::Result;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MonitorIndex(pub usize);

impl std::fmt::Display for MonitorIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Monitor {
    pub width: i32,
    pub height: i32,
}

impl Monitor {
    pub const fn new(width: i32, height: i32) -> Self {
        Monitor { width, height }
    }

    pub fn from_string_dimensions(raw_dimensions: &str) -> Result<Self> {
        let dimensions = raw_dimensions.split('x').collect::<Vec<&str>>();

        if dimensions.len() != 2 {
            return Err(anyhow::anyhow!(
                "Invalid monitor dimensions: {raw_dimensions}"
            ));
        }

        let width: i32 = dimensions[0].parse()?;
        let height: i32 = dimensions[1].parse()?;

        Ok(Monitor::new(width, height))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod monitor_index {
        use super::*;

        #[test]
        fn test_new() {
            let index = MonitorIndex(5);
            assert_eq!(index.0, 5);
        }

        #[test]
        fn test_display() {
            let index = MonitorIndex(42);
            assert_eq!(format!("{}", index), "42");
        }

        #[test]
        fn test_clone() {
            let index1 = MonitorIndex(10);
            let index2 = index1.clone();
            assert_eq!(index1.0, index2.0);
        }

        #[test]
        fn test_equality() {
            let index1 = MonitorIndex(7);
            let index2 = MonitorIndex(7);
            let index3 = MonitorIndex(8);

            assert_eq!(index1, index2);
            assert_ne!(index1, index3);
        }
    }

    mod monitor {
        use super::*;

        #[test]
        fn test_new() {
            let monitor = Monitor::new(1920, 1080);
            assert_eq!(monitor.width, 1920);
            assert_eq!(monitor.height, 1080);
        }

        #[test]
        fn test_from_string_dimensions_valid() {
            let monitor = Monitor::from_string_dimensions("1920x1080").unwrap();
            assert_eq!(monitor.width, 1920);
            assert_eq!(monitor.height, 1080);
        }

        #[test]
        fn test_from_string_dimensions_valid_large() {
            let monitor = Monitor::from_string_dimensions("3440x1440").unwrap();
            assert_eq!(monitor.width, 3440);
            assert_eq!(monitor.height, 1440);
        }

        #[test]
        fn test_from_string_dimensions_invalid_format() {
            let result = Monitor::from_string_dimensions("1920");
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("Invalid monitor dimensions: 1920"));
        }

        #[test]
        fn test_from_string_dimensions_invalid_format_too_many_parts() {
            let result = Monitor::from_string_dimensions("1920x1080x60");
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("Invalid monitor dimensions: 1920x1080x60"));
        }

        #[test]
        fn test_from_string_dimensions_invalid_numbers() {
            let result = Monitor::from_string_dimensions("widthxheight");
            assert!(result.is_err());
        }

        #[test]
        fn test_from_string_dimensions_empty() {
            let result = Monitor::from_string_dimensions("");
            assert!(result.is_err());
        }

        #[test]
        fn test_clone() {
            let monitor1 = Monitor::new(2560, 1440);
            let monitor2 = monitor1.clone();
            assert_eq!(monitor1.width, monitor2.width);
            assert_eq!(monitor1.height, monitor2.height);
        }

        #[test]
        fn test_equality() {
            let monitor1 = Monitor::new(1920, 1080);
            let monitor2 = Monitor::new(1920, 1080);
            let monitor3 = Monitor::new(2560, 1440);

            assert_eq!(monitor1, monitor2);
            assert_ne!(monitor1, monitor3);
        }
    }
}
