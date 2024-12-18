use anyhow::Result;

use super::{Monitor, Window, WINDOW_DECORATION};

pub struct MonitorGrid {
    /// A 2D array representing the arrangement of monitors. The top-level slice represents columns and each inner slice represents a row of monitors.
    /// See tests for examples.
    monitors: Vec<Vec<Monitor>>,
    /// The number of monitors in the grid.
    monitors_count: i32,
    /// The width of a single workspace (in pixels) that is made up of the monitors.
    workspace_width: i32,
    /// The height of a single workspace (in pixels) that is made up of the monitors.
    workspace_height: i32,
}

impl MonitorGrid {
    pub fn new(monitors: Vec<Vec<Monitor>>) -> Self {
        let (workspace_width, workspace_height) = Self::calculate_workspace_size(&monitors);
        let monitors_count = Self::calculate_monitor_count(&monitors);

        MonitorGrid {
            monitors,
            monitors_count,
            workspace_width,
            workspace_height,
        }
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
            && window.x_offset < self.workspace_width
            && window.y_offset >= 0
            && window.y_offset < self.workspace_height
    }

    pub fn determine_which_monitor_window_is_on(&self, window: &Window) -> Result<usize> {
        let (x_offset, y_offset) = (window.x_offset, window.y_offset);

        let mut monitor_index: i32 = -1;
        let mut row_width = 0;

        for column in &self.monitors {
            monitor_index += 1;

            let mut column_height = -(WINDOW_DECORATION);
            let mut greatest_row_width = 0;
            let base_row_width = row_width;

            for (row_index, monitor) in column.iter().enumerate() {
                monitor_index += row_index as i32;

                column_height += monitor.height;

                if monitor.width > greatest_row_width {
                    greatest_row_width = monitor.width;
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

    pub fn get_next_monitor(&self, current_monitor: i32, direction: i32) -> i32 {
        // Need to do this "multiple module operations" song and dance to get the modulo behavior we want.
        // Otherwise, we can get a negative remainder.
        //
        // Ref: https://stackoverflow.com/q/31210357
        (((current_monitor + direction) % self.monitors_count) + self.monitors_count)
            % self.monitors_count
    }

    fn calculate_monitor_count(monitors: &[Vec<Monitor>]) -> i32 {
        monitors
            .iter()
            .fold(0, |acc, column| acc + column.len() as i32)
    }

    fn calculate_workspace_size(monitors: &Vec<Vec<Monitor>>) -> (i32, i32) {
        let mut workspace_width = 0;
        let mut workspace_height = 0;

        for column in monitors {
            let mut column_height = 0;
            let mut max_column_width = 0;

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

    mod determine_which_monitor_window_is_on {
        use super::*;

        fn create_mock_window(x_offset: i32, y_offset: i32) -> Window {
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

        fn create_mock_grid() -> MonitorGrid {
            MonitorGrid::new(vec![
                vec![Monitor::new(1920, 1080), Monitor::new(1920, 1080)],
                vec![Monitor::new(3440, 1440)],
                vec![Monitor::new(1440, 2560)],
            ])
        }

        #[test]
        fn test_first_monitor() {
            let window = create_mock_window(0, 0);
            let grid = create_mock_grid();

            assert_eq!(
                grid.determine_which_monitor_window_is_on(&window).unwrap(),
                0
            );
        }

        #[test]
        fn test_second_monitor() {
            let window = create_mock_window(0, 1500);
            let grid = create_mock_grid();

            assert_eq!(
                grid.determine_which_monitor_window_is_on(&window).unwrap(),
                1
            );
        }

        #[test]
        fn test_third_monitor() {
            let window = create_mock_window(1920, 0);
            let grid = create_mock_grid();

            assert_eq!(
                grid.determine_which_monitor_window_is_on(&window).unwrap(),
                2
            );
        }

        #[test]
        fn test_fourth_monitor() {
            let window = create_mock_window(5364, 0);
            let grid = create_mock_grid();

            assert_eq!(
                grid.determine_which_monitor_window_is_on(&window).unwrap(),
                3
            );
        }

        #[test]
        fn test_invalid_monitor() {
            let window = create_mock_window(100000, 0);
            let grid = create_mock_grid();

            assert!(grid.determine_which_monitor_window_is_on(&window).is_err());
        }
    }

    mod calculate_workspace_size {
        use super::*;

        #[test]
        fn test_my_arrangement() {
            let monitors = vec![
                vec![Monitor::new(1920, 1080), Monitor::new(1920, 1080)],
                vec![Monitor::new(3440, 1440)],
                vec![Monitor::new(1440, 2560)],
            ];

            let (workspace_width, workspace_height) =
                MonitorGrid::calculate_workspace_size(&monitors);

            assert_eq!(workspace_width, 1920 + 3440 + 1440);
            assert_eq!(workspace_height, 2560); // The max height of all columns
        }

        #[test]
        fn test_different_arrangement() {
            let monitors = vec![
                vec![Monitor::new(1920, 1080)],
                vec![Monitor::new(1440, 3440)],
                vec![Monitor::new(1440, 2560)],
            ];

            let (workspace_width, workspace_height) =
                MonitorGrid::calculate_workspace_size(&monitors);

            assert_eq!(workspace_width, 1920 + 1440 + 1440);
            assert_eq!(workspace_height, 3440); // The max height of all columns
        }

        #[test]
        fn test_single_monitor() {
            let monitors = vec![vec![Monitor::new(1920, 1080)]];

            let (workspace_width, workspace_height) =
                MonitorGrid::calculate_workspace_size(&monitors);

            assert_eq!(workspace_width, 1920);
            assert_eq!(workspace_height, 1080);
        }

        #[test]
        fn test_empty_arrangement() {
            let monitors: Vec<Vec<Monitor>> = vec![];

            let (workspace_width, workspace_height) =
                MonitorGrid::calculate_workspace_size(&monitors);

            assert_eq!(workspace_width, 0);
            assert_eq!(workspace_height, 0);
        }
    }
}
