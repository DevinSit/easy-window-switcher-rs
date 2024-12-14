/// Models the position of a single Workspace within the plane of a WorkspaceGrid.
/// Specifically, the x/y coordinates represent the top-leftmost pixel of a Workspace.
///
/// A "Workspace" is a direct mapping to the Ubuntu concept of a 'workspace' (or Windows' concept of a 'virtual desktop').
/// That is, the "current Workspace" is all of the currently visible screen real-estate from all of the monitors connected to a computer.
/// Switching to a different Workspace (e.g. in my 3x3 grid of Workspaces) means that the monitors will display a different set of windows.
///
/// The "Workspace" is modelled as its top-leftmost pixel because of how `wmctrl` represents the entire plane of screen real-estate (i.e. the WorkspaceGrid).
///
/// That is, given my 3x3 grid of Workspaces (i.e. a 3x3 WorkspaceGrid) with 3 horizontally laid out 1920x1080 monitors, you end up with a single
/// Workspace being represented by the total size of the monitors (i.e. 1920*3 x 1080*1 = 5760x1080) and the WorkspaceGrid being represented by
/// the total size of all Workspaces (i.e. for a 3x3 grid, 5760*3 x 1080*3 = 17280x3240).
///
/// Therefore, in this 17280x3240 grid of Workspaces, each of the 9 Workspaces would be represented by the following coordinates:
///
/// X         Y       Index
/// 0,        0       0
/// 5760      0       1
/// 11520     0       2
/// 0         1080    3
/// 5760      1080    4
/// 11520     1080    5
/// 0         2160    6
/// 5760      2160    7
/// 11520     2160    8
///
/// We need to know what the coordinates are of a Workspace since the positions of Windows within the WorkspaceGrid are relative to the entire plane,
/// _not_ relative to the current Workspace (i.e. a window with coordinates of (7680, 0) is on the second Workspace, middle monitor assuming the example grid above).
#[derive(Debug)]
pub struct Workspace {
    x: i32,
    y: i32,
}

impl Workspace {
    /// Creates a new `Workspace` with the given dimensions.
    fn new(x: i32, y: i32) -> Self {
        Workspace { x, y }
    }

    /// Creates a new `Workspace` from a raw configuration string, e.g. "1920,1080".
    fn from_raw_config(raw_dimensions: &str) -> Self {
        let split_dimensions = parse_dimensions(raw_dimensions);

        Workspace {
            x: split_dimensions[0],
            y: split_dimensions[1],
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
        assert_eq!(workspace.x, 1920);
        assert_eq!(workspace.y, 1080);
    }

    #[test]
    fn test_from_raw_config() {
        let workspace = Workspace::from_raw_config("1920,1080");
        assert_eq!(workspace.x, 1920);
        assert_eq!(workspace.y, 1080);
    }
}
