use anyhow::Result;

use super::{Monitor, Window, WINDOW_DECORATION};

// TODO: Rewrite this comment.

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

type MonitorArrangement = Vec<Vec<Monitor>>;

/// A 2D array representing the arrangement of monitors. The top-level slice represents columns and each inner slice represents a row of monitors.
#[rustfmt::skip] // Note: Ignore rustfmt so that we can better visually see the monitor arrangement.
const MONITOR_ARRANGEMENT: &[&[Monitor]] =
    &[
        &[Monitor::new(1920, 1080),
          Monitor::new(1920, 1080)], &[Monitor::new(3440, 1440)], &[Monitor::new(1440, 2560)]
    ];

const GRID_ROWS_COUNT: usize = 3;
const GRID_COLUMNS_COUNT: usize = 3;

#[derive(Clone, Debug, Default)]
pub struct WorkspaceGrid {
    pub width: usize,  // pixels
    pub height: usize, // pixels
    rows_count: usize,
    columns_count: usize,
    monitor_arrangement: MonitorArrangement,
    workspace_width: usize,  // pixels
    workspace_height: usize, // pixels
}

impl WorkspaceGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let monitor_arrangement: MonitorArrangement = MONITOR_ARRANGEMENT
            .iter()
            .map(|&monitors| monitors.to_vec())
            .collect();

        let (workspace_width, workspace_height) =
            WorkspaceGrid::calculate_workspace_size(&monitor_arrangement);

        WorkspaceGrid {
            width,
            height,
            rows_count: GRID_ROWS_COUNT,
            columns_count: GRID_COLUMNS_COUNT,
            monitor_arrangement,
            workspace_width,
            workspace_height,
        }
    }

    pub fn from_string_dimensions(string_dimensions: &str) -> Self {
        let grid_dimensions = string_dimensions
            .trim()
            .split("x")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        WorkspaceGrid::new(grid_dimensions[0], grid_dimensions[1])
    }

    pub fn is_window_in_current_workspace(&self, window: &Window) -> bool {
        // Can find the windows in the current workspace by looking at the x and y offsets.
        //
        // Negative offsets mean that the window is placed somewhere outside of the current workspace.
        //
        // Therefore, if x-offset isn't negative, the y-offset isn't negative,
        // the x-offset doesn't exceed the total width of the workspace,
        // and the y-offset doesn't exceed the total height of the workspace,
        // then the window is in the current workspace.

        window.x_offset >= 0
            && window.x_offset < self.workspace_width as i32
            && window.y_offset >= 0
            && window.y_offset < self.workspace_height as i32
    }

    pub fn determine_which_monitor_window_is_on(&self, window: &Window) -> Result<usize> {
        let (x_offset, y_offset) = (window.x_offset, window.y_offset);

        let mut monitor_index: i32 = -1;
        let mut row_width = 0;

        for column in &self.monitor_arrangement {
            monitor_index += 1;
            let mut column_height = -(WINDOW_DECORATION);
            let mut greatest_row_width = 0;
            let base_row_width = row_width;

            for (row_index, monitor) in column.iter().enumerate() {
                monitor_index += row_index as i32;

                column_height += monitor.height as i32;

                if monitor.width as i32 > greatest_row_width {
                    greatest_row_width = monitor.width as i32;
                    row_width = base_row_width + greatest_row_width;
                }

                if x_offset < row_width && y_offset < column_height {
                    return Ok(monitor_index as usize);
                }
            }
        }

        Err(anyhow::anyhow!(
            "Window is not on any monitor; position x {x_offset}, y {y_offset}"
        ))
    }

    fn calculate_workspace_size(monitor_arrangement: &MonitorArrangement) -> (usize, usize) {
        let mut workspace_width: usize = 0;
        let mut workspace_height: usize = 0;

        for column in monitor_arrangement {
            let mut column_height: usize = 0;
            let mut max_column_width: usize = 0;

            for monitor in column {
                column_height += monitor.height;

                if monitor.width > max_column_width {
                    max_column_width = monitor.width;
                }
            }

            if column_height > workspace_height {
                workspace_height = column_height;
            }

            workspace_width += max_column_width;
        }

        (workspace_width, workspace_height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod calculate_workspace_size {
        use super::*;

        #[test]
        fn test_my_arrangement() {
            let monitor_arrangement = vec![
                vec![Monitor::new(1920, 1080), Monitor::new(1920, 1080)],
                vec![Monitor::new(3440, 1440)],
                vec![Monitor::new(1440, 2560)],
            ];

            let (workspace_width, workspace_height) =
                WorkspaceGrid::calculate_workspace_size(&monitor_arrangement);

            assert_eq!(workspace_width, 1920 + 3440 + 1440);
            assert_eq!(workspace_height, 2560); // The max height of all columns
        }

        #[test]
        fn test_different_arrangement() {
            let monitor_arrangement = vec![
                vec![Monitor::new(1920, 1080)],
                vec![Monitor::new(1440, 3440)],
                vec![Monitor::new(1440, 2560)],
            ];

            let (workspace_width, workspace_height) =
                WorkspaceGrid::calculate_workspace_size(&monitor_arrangement);

            assert_eq!(workspace_width, 1920 + 1440 + 1440);
            assert_eq!(workspace_height, 3440); // The max height of all columns
        }

        #[test]
        fn test_single_monitor() {
            let monitor_arrangement = vec![vec![Monitor::new(1920, 1080)]];

            let (workspace_width, workspace_height) =
                WorkspaceGrid::calculate_workspace_size(&monitor_arrangement);

            assert_eq!(workspace_width, 1920);
            assert_eq!(workspace_height, 1080);
        }

        #[test]
        fn test_empty_arrangement() {
            let monitor_arrangement: MonitorArrangement = vec![];

            let (workspace_width, workspace_height) =
                WorkspaceGrid::calculate_workspace_size(&monitor_arrangement);

            assert_eq!(workspace_width, 0);
            assert_eq!(workspace_height, 0);
        }
    }

    mod determine_which_monitor_window_is_on {
        use super::*;

        fn create_window(x_offset: i32, y_offset: i32) -> Window {
            // Only values that matter are the offsets; everything else can be arbitrary.
            Window {
                id: 1,
                x_offset,
                y_offset,
                width: 1920,
                height: 1056,
                window_class: "chrome".to_string(),
                title: "Chrome".to_string(),
            }
        }

        #[test]
        fn test_first_monitor() {
            let window = create_window(0, 0);
            let grid = WorkspaceGrid::new(0, 0);

            assert_eq!(
                grid.determine_which_monitor_window_is_on(&window).unwrap(),
                0
            );
        }

        #[test]
        fn test_second_monitor() {
            let window = create_window(0, 1500);
            let grid = WorkspaceGrid::new(0, 0);

            assert_eq!(
                grid.determine_which_monitor_window_is_on(&window).unwrap(),
                1
            );
        }

        #[test]
        fn test_third_monitor() {
            let window = create_window(1920, 0);
            let grid = WorkspaceGrid::new(0, 0);

            assert_eq!(
                grid.determine_which_monitor_window_is_on(&window).unwrap(),
                2
            );
        }

        #[test]
        fn test_fourth_monitor() {
            let window = create_window(5364, 0);
            let grid = WorkspaceGrid::new(0, 0);

            assert_eq!(
                grid.determine_which_monitor_window_is_on(&window).unwrap(),
                3
            );
        }

        #[test]
        fn test_invalid_monitor() {
            let window = create_window(100000, 0);
            let grid = WorkspaceGrid::new(0, 0);

            assert!(grid.determine_which_monitor_window_is_on(&window).is_err());
        }
    }
}
