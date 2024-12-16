use super::{monitor::Monitor, WorkspacePosition};

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
type WorkspaceIndices = Vec<Vec<usize>>;

/// A 2D array representing the arrangement of monitors. The top-level slice represents columns and each inner slice represents a row of monitors.
#[rustfmt::skip] // Note: Ignore rustfmt so that we can better visually see the monitor arrangement.
const MONITOR_ARRANGEMENT: &[&[Monitor]] =
    &[
        &[Monitor {width: 1920, height: 1080},
          Monitor {width: 1920, height: 1080}], &[Monitor {width: 3440, height: 1080}], &[Monitor {width: 1440, height: 2560}]
    ];

const GRID_ROWS_COUNT: usize = 3;
const GRID_COLUMNS_COUNT: usize = 3;

#[derive(Clone, Debug, Default)]
pub struct WorkspaceGrid {
    width: usize,  // pixels
    height: usize, // pixels
    rows_count: usize,
    columns_count: usize,
    monitor_arrangement: MonitorArrangement,
    workspace_width: usize,  // pixels
    workspace_height: usize, // pixels
    workspace_indices: WorkspaceIndices,
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
            workspace_indices: WorkspaceGrid::calculate_workspace_indices(
                GRID_COLUMNS_COUNT,
                GRID_COLUMNS_COUNT,
            ),
        }
    }

    pub fn get_workspace_index(&self, workspace_position: WorkspacePosition) -> usize {
        WorkspaceGrid::calculate_workspace_index(
            &self.workspace_indices,
            self.workspace_width,
            self.workspace_height,
            workspace_position,
        )
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

    fn calculate_workspace_indices(rows_count: usize, columns_count: usize) -> WorkspaceIndices {
        let mut workspace_indices: WorkspaceIndices = Vec::new();

        for row in 0..rows_count {
            workspace_indices.push(Vec::new());

            for column in 0..columns_count {
                workspace_indices[row].push((row * columns_count) + column);
            }
        }

        workspace_indices
    }

    fn calculate_workspace_index(
        workspace_indices: &WorkspaceIndices,
        workspace_width: usize,
        workspace_height: usize,
        workspace_position: WorkspacePosition,
    ) -> usize {
        let row_index = workspace_position.x / workspace_width;
        let column_index = workspace_position.y / workspace_height;

        workspace_indices[column_index][row_index]
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

    mod calculate_workspace_indices {
        use super::*;

        #[test]
        fn test_3x3_grid() {
            let indices = WorkspaceGrid::calculate_workspace_indices(3, 3);

            // Expected workspace indices for a 3x3 grid
            let expected_indices = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];

            assert_eq!(indices, expected_indices);
        }

        #[test]
        fn test_2x2_grid() {
            let indices = WorkspaceGrid::calculate_workspace_indices(2, 2);

            // Expected workspace indices for a 2x2 grid
            let expected_indices = vec![vec![0, 1], vec![2, 3]];

            assert_eq!(indices, expected_indices);
        }

        #[test]
        fn test_non_square_grid() {
            let indices = WorkspaceGrid::calculate_workspace_indices(3, 2);

            // Expected workspace indices for a 3x2 grid
            let expected_indices = vec![vec![0, 1], vec![2, 3], vec![4, 5]];

            assert_eq!(indices, expected_indices);
        }

        #[test]
        fn test_single_row() {
            let indices = WorkspaceGrid::calculate_workspace_indices(1, 6);

            // Expected workspace indices for a single row with 6 columns
            let expected_indices = vec![vec![0, 1, 2, 3, 4, 5]];

            assert_eq!(indices, expected_indices);
        }

        #[test]
        fn test_single_column() {
            let indices = WorkspaceGrid::calculate_workspace_indices(4, 1);

            // Expected workspace indices for a single column with 4 rows
            let expected_indices = vec![vec![0], vec![1], vec![2], vec![3]];

            assert_eq!(indices, expected_indices);
        }

        #[test]
        fn test_empty_grid() {
            let indices = WorkspaceGrid::calculate_workspace_indices(1, 1);

            // Expected workspace indices for an empty grid (but still a 1x1 grid)
            let expected_indices = vec![vec![0]];

            assert_eq!(indices, expected_indices);
        }
    }

    mod calculate_workspace_index {
        use super::*;

        #[test]
        fn test_top_left() {
            let workspace_indices = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
            let workspace_width = 640;
            let workspace_height = 480;
            let position = WorkspacePosition { x: 0, y: 0 };

            let index = WorkspaceGrid::calculate_workspace_index(
                &workspace_indices,
                workspace_width,
                workspace_height,
                position,
            );

            assert_eq!(index, 0);
        }

        #[test]
        fn test_top_right() {
            let workspace_indices = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
            let workspace_width = 640;
            let workspace_height = 480;
            let position = WorkspacePosition { x: 1280, y: 0 };

            let index = WorkspaceGrid::calculate_workspace_index(
                &workspace_indices,
                workspace_width,
                workspace_height,
                position,
            );

            assert_eq!(index, 2);
        }

        #[test]
        fn test_bottom_left() {
            let workspace_indices = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
            let workspace_width = 640;
            let workspace_height = 480;
            let position = WorkspacePosition { x: 0, y: 960 };

            let index = WorkspaceGrid::calculate_workspace_index(
                &workspace_indices,
                workspace_width,
                workspace_height,
                position,
            );

            assert_eq!(index, 6);
        }

        #[test]
        fn test_bottom_right() {
            let workspace_indices = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
            let workspace_width = 640;
            let workspace_height = 480;
            let position = WorkspacePosition { x: 1280, y: 960 };

            let index = WorkspaceGrid::calculate_workspace_index(
                &workspace_indices,
                workspace_width,
                workspace_height,
                position,
            );

            assert_eq!(index, 8);
        }

        #[test]
        fn test_center() {
            let workspace_indices = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
            let workspace_width = 640;
            let workspace_height = 480;
            let position = WorkspacePosition { x: 640, y: 480 };

            let index = WorkspaceGrid::calculate_workspace_index(
                &workspace_indices,
                workspace_width,
                workspace_height,
                position,
            );

            assert_eq!(index, 4);
        }
    }
}
