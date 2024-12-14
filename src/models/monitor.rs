/// Models the attributes (specifically, dimensions) of a single monitor.
/// Many Monitors form a MonitorGrid.
///
/// Note that Monitor dimensions represent the top-leftmost pixel of a monitor.
/// e.g. 0,0 is the first monitor, 3840,0 is the second monitor for 3 1920x1080 monitors, etc.
#[derive(Debug)]
pub struct Monitor {
    width: i32,
    height: i32,
}

impl Monitor {
    /// Creates a new `Monitor` with the given dimensions.
    fn new(width: i32, height: i32) -> Self {
        Monitor { width, height }
    }

    /// Creates a new `Monitor` from a raw configuration string, e.g. "1920,1080".
    fn from_raw_config(raw_dimensions: &str) -> Self {
        let split_dimensions = parse_dimensions(raw_dimensions);

        Monitor {
            width: split_dimensions[0],
            height: split_dimensions[1],
        }
    }
}

/// Helper function to parse dimensions from a string.
fn parse_dimensions(raw_dimensions: &str) -> Vec<i32> {
    raw_dimensions
        .split(',')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect()
}

// src/models/monitor.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let monitor = Monitor::new(1920, 1080);
        assert_eq!(monitor.width, 1920);
        assert_eq!(monitor.height, 1080);
    }

    #[test]
    fn test_from_raw_config() {
        let monitor = Monitor::from_raw_config("1920,1080");
        assert_eq!(monitor.width, 1920);
        assert_eq!(monitor.height, 1080);
    }

    #[test]
    fn test_parse_dimensions() {
        let dimensions = parse_dimensions("1920,1080");
        assert_eq!(dimensions[0], 1920);
        assert_eq!(dimensions[1], 1080);
    }
}
