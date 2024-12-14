/// Models the attributes (specifically, dimensions) of a single Workspace.
/// Many Workspaces form a WorkspaceGrid.
///
/// Note that Workspace dimensions represent the top-leftmost pixel of a workspace.
/// e.g. in a 3x3 grid with 3 1920x1080 monitors:
/// - 0,0 is the first workspace,
/// - 5760,0 is the second workspace, etc
#[derive(Debug)]
pub struct Workspace {
    width: i32,
    height: i32,
}

impl Workspace {
    /// Creates a new `Workspace` with the given dimensions.
    fn new(width: i32, height: i32) -> Self {
        Workspace { width, height }
    }

    /// Creates a new `Workspace` from a raw configuration string, e.g. "1920,1080".
    fn from_raw_config(raw_dimensions: &str) -> Self {
        let split_dimensions = parse_dimensions(raw_dimensions);

        Workspace {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let workspace = Workspace::new(1920, 1080);
        assert_eq!(workspace.width, 1920);
        assert_eq!(workspace.height, 1080);
    }

    #[test]
    fn test_from_raw_config() {
        let workspace = Workspace::from_raw_config("1920,1080");
        assert_eq!(workspace.width, 1920);
        assert_eq!(workspace.height, 1080);
    }

    #[test]
    fn test_parse_dimensions() {
        let dimensions = parse_dimensions("1920,1080");
        assert_eq!(dimensions[0], 1920);
        assert_eq!(dimensions[1], 1080);
    }
}
