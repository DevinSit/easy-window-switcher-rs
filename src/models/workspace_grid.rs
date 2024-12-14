use super::Window;

/// These are assumptions about how the user's workspaces are setup (based on their monitors).
///
/// For example, three horizontally-aligned 1920x1080 monitors would have a single workspace dimension of:
///
/// WORKSPACE_HEIGHT = 1 x 1080
/// WORKSPACE_WIDTH = 3 x 1920
///
/// For the values below, they are calculated using a quad-monitor setup that looks like this:
///
/// [1920x1080]
///               [3440x1440]     [1440x2560]
/// [1920x1080]
///
/// That is, two vertically stacked 1080p monitors on the left, with a 3440x1440 ultrawide in the middle,
/// and a portrait 2560x1440 monitor on the right.
///
/// With this setup, these values are calculated as follows:
///
/// WORKSPACE_HEIGHT = 2560 (aka the max height of all the monitors)
/// WORKSPACE_WIDTH = 1920 + 3440 + 1440
pub const WORKSPACE_HEIGHT: i32 = 2560;
pub const WORKSPACE_WIDTH: i32 = 6800;

pub const WORKSPACE_HORIZONTAL_COUNT: i32 = 3;
pub const WORKSPACE_VERTICAL_COUNT: i32 = 3;

#[derive(Debug)]
pub struct WorkspaceGrid {
    width: i32,
    height: i32,
    workspace_width: i32,
    workspace_height: i32,
    workspace_horizontal_count: i32,
    workspace_vertical_count: i32,
}
