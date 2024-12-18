use super::{MonitorGrid, Window};

pub struct Workspace {
    /// A 2D array representing the arrangement of monitors. The top-level slice represents columns and each inner slice represents a row of monitors.
    /// See tests for examples.
    pub monitor_grid: MonitorGrid,

    /// The width of a single workspace (in pixels) that is made up of the monitors.
    workspace_width: i32,

    /// The height of a single workspace (in pixels) that is made up of the monitors.
    workspace_height: i32,
}

impl Workspace {
    pub fn new(monitor_grid: MonitorGrid) -> Self {
        let (workspace_width, workspace_height) = Self::calculate_workspace_size(&monitor_grid);

        Workspace {
            monitor_grid,
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

    fn calculate_workspace_size(monitor_grid: &MonitorGrid) -> (i32, i32) {
        let mut workspace_width = 0;
        let mut workspace_height = 0;

        for column in monitor_grid.0.iter() {
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
    use crate::models::Monitor;

    mod calculate_workspace_size {
        use super::*;

        #[test]
        fn test_my_arrangement() {
            let monitor_grid = MonitorGrid(vec![
                vec![Monitor::new(1920, 1080), Monitor::new(1920, 1080)],
                vec![Monitor::new(3440, 1440)],
                vec![Monitor::new(1440, 2560)],
            ]);

            let (workspace_width, workspace_height) =
                Workspace::calculate_workspace_size(&monitor_grid);

            assert_eq!(workspace_width, 1920 + 3440 + 1440);
            assert_eq!(workspace_height, 2560); // The max height of all columns
        }

        #[test]
        fn test_different_arrangement() {
            let monitor_grid = MonitorGrid(vec![
                vec![Monitor::new(1920, 1080)],
                vec![Monitor::new(1440, 3440)],
                vec![Monitor::new(1440, 2560)],
            ]);

            let (workspace_width, workspace_height) =
                Workspace::calculate_workspace_size(&monitor_grid);

            assert_eq!(workspace_width, 1920 + 1440 + 1440);
            assert_eq!(workspace_height, 3440); // The max height of all columns
        }

        #[test]
        fn test_single_monitor() {
            let monitor_grid = MonitorGrid(vec![vec![Monitor::new(1920, 1080)]]);

            let (workspace_width, workspace_height) =
                Workspace::calculate_workspace_size(&monitor_grid);

            assert_eq!(workspace_width, 1920);
            assert_eq!(workspace_height, 1080);
        }

        #[test]
        fn test_empty_arrangement() {
            let monitor_grid = MonitorGrid(vec![]);

            let (workspace_width, workspace_height) =
                Workspace::calculate_workspace_size(&monitor_grid);

            assert_eq!(workspace_width, 0);
            assert_eq!(workspace_height, 0);
        }
    }
}
