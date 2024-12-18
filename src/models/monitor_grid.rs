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

    pub fn determine_which_monitor_window_is_on(&self, window: &Window) -> Result<usize> {
        let (x_offset, y_offset) = (window.x_offset, window.y_offset);

        let mut monitor_index: i32 = -1;
        let mut row_width = 0;

        for column in &self.0 {
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
}
