use derive_builder::Builder;

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

#[derive(Clone, Debug)]
struct Monitor {
    width: i32,
    height: i32,
}

type MonitorArrangement = Vec<Vec<Monitor>>;

/// A 2D array representing the arrangement of monitors. The top-level slice represents columns and each inner slice represents a row of monitors.
#[rustfmt::skip] // Note: Ignore rustfmt so that we can better visually see the monitor arrangement.
const MONITOR_ARRANGEMENT: &[&[Monitor]] =
    &[
        &[Monitor {width: 1920, height: 1080},
          Monitor {width: 1920, height: 1080}], &[Monitor {width: 3440, height: 1080}], &[Monitor {width: 1440, height: 2560}]
    ];

const GRID_ROWS_COUNT: i32 = 3;
const GRID_COLUMNS_COUNT: i32 = 3;

#[derive(Builder, Clone, Debug, Default)]
#[builder(default)]
pub struct WorkspaceGrid {
    width: i32,  // pixels
    height: i32, // pixels
    rows_count: i32,
    columns_count: i32,
    monitor_arrangement: MonitorArrangement,
    workspace_width: i32,  // pixels
    workspace_height: i32, // pixels
}

impl WorkspaceGrid {
    pub fn new(width: i32, height: i32) -> Self {
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

    fn calculate_workspace_size(monitor_arrangement: &MonitorArrangement) -> (i32, i32) {
        let mut workspace_width = 0;
        let mut workspace_height = 0;

        for column in monitor_arrangement {
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

    mod calculate_workspace_size {
        use super::*;

        #[test]
        fn test_my_arrangement() {
            let monitor_arrangement = vec![
                vec![
                    Monitor {
                        width: 1920,
                        height: 1080,
                    },
                    Monitor {
                        width: 1920,
                        height: 1080,
                    },
                ],
                vec![Monitor {
                    width: 3440,
                    height: 1440,
                }],
                vec![Monitor {
                    width: 1440,
                    height: 2560,
                }],
            ];

            let (workspace_width, workspace_height) =
                WorkspaceGrid::calculate_workspace_size(&monitor_arrangement);

            assert_eq!(workspace_width, 1920 + 3440 + 1440);
            assert_eq!(workspace_height, 2560); // The max height of all columns
        }

        #[test]
        fn test_different_arrangement() {
            let monitor_arrangement = vec![
                vec![Monitor {
                    width: 1920,
                    height: 1080,
                }],
                vec![Monitor {
                    width: 1440,
                    height: 3440,
                }],
                vec![Monitor {
                    width: 1440,
                    height: 2560,
                }],
            ];

            let (workspace_width, workspace_height) =
                WorkspaceGrid::calculate_workspace_size(&monitor_arrangement);

            assert_eq!(workspace_width, 1920 + 1440 + 1440);
            assert_eq!(workspace_height, 3440); // The max height of all columns
        }

        #[test]
        fn test_single_monitor() {
            let monitor_arrangement = vec![vec![Monitor {
                width: 1920,
                height: 1080,
            }]];

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
}
