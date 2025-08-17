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

    mod new {
        use super::*;

        #[test]
        fn test_new() {
            let monitor_grid = MonitorGrid(vec![
                vec![Monitor::new(1920, 1080), Monitor::new(1920, 1080)],
                vec![Monitor::new(3440, 1440)],
            ]);

            let workspace = Workspace::new(monitor_grid.clone());

            assert_eq!(workspace.workspace_width, 1920 + 3440);
            assert_eq!(workspace.workspace_height, 2160); // max of (1080+1080=2160, 1440)
        }

        #[test]
        fn test_new_single_monitor() {
            let monitor_grid = MonitorGrid(vec![vec![Monitor::new(2560, 1440)]]);
            let workspace = Workspace::new(monitor_grid);

            assert_eq!(workspace.workspace_width, 2560);
            assert_eq!(workspace.workspace_height, 1440);
        }

        #[test]
        fn test_new_empty_grid() {
            let monitor_grid = MonitorGrid(vec![]);
            let workspace = Workspace::new(monitor_grid);

            assert_eq!(workspace.workspace_width, 0);
            assert_eq!(workspace.workspace_height, 0);
        }
    }

    mod is_window_in_current_workspace {
        use super::*;
        use crate::models::{Window, WindowId};

        fn create_test_workspace() -> Workspace {
            let monitor_grid = MonitorGrid(vec![
                vec![Monitor::new(1920, 1080)],
                vec![Monitor::new(1920, 1080)],
            ]);
            Workspace::new(monitor_grid)
        }

        fn create_test_window(x_offset: i32, y_offset: i32) -> Window {
            Window {
                id: WindowId(1),
                x_offset,
                y_offset,
                width: 800,
                height: 600,
                window_class: "test".to_string(),
                title: "Test Window".to_string(),
            }
        }

        #[test]
        fn test_window_in_workspace() {
            let workspace = create_test_workspace();
            let window = create_test_window(100, 100);

            assert!(workspace.is_window_in_current_workspace(&window));
        }

        #[test]
        fn test_window_at_origin() {
            let workspace = create_test_workspace();
            let window = create_test_window(0, 0);

            assert!(workspace.is_window_in_current_workspace(&window));
        }

        #[test]
        fn test_window_at_edge() {
            let workspace = create_test_workspace();
            // Workspace is 3840x1080 (1920+1920 x max(1080))
            let window = create_test_window(3839, 1079);

            assert!(workspace.is_window_in_current_workspace(&window));
        }

        #[test]
        fn test_window_negative_x() {
            let workspace = create_test_workspace();
            let window = create_test_window(-1, 100);

            assert!(!workspace.is_window_in_current_workspace(&window));
        }

        #[test]
        fn test_window_negative_y() {
            let workspace = create_test_workspace();
            let window = create_test_window(100, -1);

            assert!(!workspace.is_window_in_current_workspace(&window));
        }

        #[test]
        fn test_window_exceeds_width() {
            let workspace = create_test_workspace();
            // Workspace width is 3840
            let window = create_test_window(3840, 100);

            assert!(!workspace.is_window_in_current_workspace(&window));
        }

        #[test]
        fn test_window_exceeds_height() {
            let workspace = create_test_workspace();
            // Workspace height is 1080
            let window = create_test_window(100, 1080);

            assert!(!workspace.is_window_in_current_workspace(&window));
        }

        #[test]
        fn test_window_far_outside() {
            let workspace = create_test_workspace();
            let window = create_test_window(10000, 10000);

            assert!(!workspace.is_window_in_current_workspace(&window));
        }

        #[test]
        fn test_window_negative_both() {
            let workspace = create_test_workspace();
            let window = create_test_window(-100, -100);

            assert!(!workspace.is_window_in_current_workspace(&window));
        }
    }
}
