use anyhow::Result;

use super::{FocusDirection, Monitor, MonitorIndex, Window, WINDOW_DECORATION};

pub struct MonitorGrid(pub Vec<Vec<Monitor>>);

impl MonitorGrid {
    pub fn get_next_monitor(
        &self,
        current_monitor: &MonitorIndex,
        direction: &FocusDirection,
    ) -> MonitorIndex {
        let monitors_count = self.calculate_monitor_count();

        MonitorIndex(
            // Need to do this "multiple module operations" song and dance to get the modulo behavior we want.
            // Otherwise, we can get a negative remainder.
            //
            // Ref: https://stackoverflow.com/q/31210357
            ((((current_monitor.0 as i32 + direction.to_int()) % monitors_count) + monitors_count)
                % monitors_count) as usize,
        )
    }

    /// Given a window (with its position via the x and y offsets), determines which monitor it is on within the grid.
    ///
    /// The algorithm intuitively works follows: for each monitor, check if the window's x/y offsets shows that it's within the bounds of the monitor's size.
    /// Calculate this by accumulating the width of all previous monitors as each column is checked, and similarly with the height of all previous monitors as each column is checked.
    pub fn determine_which_monitor_window_is_on(&self, window: &Window) -> Result<MonitorIndex> {
        // This is the index of the monitor that the monitor is on (0-indexed).
        // Start it at negative one since each loop through the monitors will increment it by one.
        let mut monitor_index: i32 = -1;

        // This is the accumulated current x position after processing each monitor.
        // Each column of monitors will have its width added to this (the widest monitor of each column only).
        let mut x_position = 0;

        for column in &self.0 {
            monitor_index += 1;

            // This is the accumulated current y position after processing each monitor in the current column.
            // Because of how the grid is represented (rows then columns), this value only needs to be accumulated once per column.
            //
            // Start it with negative WINDOW_DECORATION so that we don't have to subtract it out later.
            let mut y_position = -(WINDOW_DECORATION);

            // Tracks which monitors in the current column has the greatest width, so that we can calculate x_position for the next column correctly.
            let mut greatest_column_width = 0;

            // Tracks the x_position coming into the column to use as a base for calculations within the column.
            let base_x_position = x_position;

            for (row_index, monitor) in column.iter().enumerate() {
                // Add the current row in the column to the index.
                //
                // Note: Adding 0 for the first index in a column is intentional, since it's handled by the increment that happens in the column loop above.
                monitor_index += row_index as i32;

                // Accumulate the current column's y position based on the monitor's height.
                y_position += monitor.height;

                if monitor.width > greatest_column_width {
                    // Update the greatest width if the current monitor is wider than the last one in the column.
                    greatest_column_width = monitor.width;

                    // Also update the overall x_position based on the new greatest width.
                    x_position = base_x_position + greatest_column_width;
                }

                // Check if the window is on the monitor by comparing the x/y positions of the monitor with the x/y offsets of the window.
                //
                // Note that the "less than" checks only work here because of how we're accumulating the positions of the monitors by checking
                // each monitor _in order_. If we weren't doing it in order, we wouldn't be able to ignore previous monitors and would have to
                // do a bounds check based on each monitor's dimensions.
                if window.x_offset < x_position && window.y_offset < y_position {
                    return Ok(MonitorIndex(monitor_index as usize));
                }
            }
        }

        Err(anyhow::anyhow!(
            "Window is not on any monitor; position x {}, y {}",
            window.x_offset,
            window.y_offset
        ))
    }

    fn calculate_monitor_count(&self) -> i32 {
        self.0
            .iter()
            .fold(0, |acc, column| acc + column.len() as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod determine_which_monitor_window_is_on {
        use super::*;

        use crate::models::WindowId;

        fn create_mock_window(x_offset: i32, y_offset: i32) -> Window {
            // Only values that matter are the offsets; everything else can be arbitrary.
            Window {
                id: WindowId(1),
                x_offset,
                y_offset,
                width: 1920,
                height: 1056,
                window_class: "chrome".to_string(),
                title: "Chrome".to_string(),
            }
        }

        fn create_mock_grid() -> MonitorGrid {
            MonitorGrid(vec![
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
                MonitorIndex(0)
            );
        }

        #[test]
        fn test_second_monitor() {
            let window = create_mock_window(0, 1500);
            let grid = create_mock_grid();

            assert_eq!(
                grid.determine_which_monitor_window_is_on(&window).unwrap(),
                MonitorIndex(1)
            );
        }

        #[test]
        fn test_third_monitor() {
            let window = create_mock_window(1920, 0);
            let grid = create_mock_grid();

            assert_eq!(
                grid.determine_which_monitor_window_is_on(&window).unwrap(),
                MonitorIndex(2)
            );
        }

        #[test]
        fn test_fourth_monitor() {
            let window = create_mock_window(5364, 0);
            let grid = create_mock_grid();

            assert_eq!(
                grid.determine_which_monitor_window_is_on(&window).unwrap(),
                MonitorIndex(3)
            );
        }

        #[test]
        fn test_invalid_monitor() {
            let window = create_mock_window(100000, 0);
            let grid = create_mock_grid();

            assert!(grid.determine_which_monitor_window_is_on(&window).is_err());
        }
    }
}
