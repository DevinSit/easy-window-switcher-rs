use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::models::{FocusDirection, MonitorIndex};
use crate::services::window_focuser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Focuses onto the closest window in the given direction; wraps around until a window is found.
    Direction {
        /// Valid directions are [left, right].
        direction: String,
    },
    /// Focuses onto the window on the monitor with the given index.
    Monitor {
        /// The index is 0-based and increases from left-to-right.
        monitor: usize,
    },
}

pub fn run() -> Result<()> {
    let args = Args::parse();

    match args.cmd {
        Commands::Direction { direction } => {
            window_focuser::focus_by_direction(FocusDirection::try_from(direction)?)
        }
        Commands::Monitor { monitor } => {
            window_focuser::focus_by_monitor_index(MonitorIndex(monitor))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parsing_direction_left() {
        // Note: This test focuses on the parsing logic rather than actual command execution
        // since the window_focuser functions have external dependencies
        let direction = "left";
        let focus_direction = FocusDirection::try_from(direction).unwrap();
        assert_eq!(focus_direction, FocusDirection::Left);
    }

    #[test]
    fn test_args_parsing_direction_right() {
        let direction = "right";
        let focus_direction = FocusDirection::try_from(direction).unwrap();
        assert_eq!(focus_direction, FocusDirection::Right);
    }

    #[test]
    fn test_args_parsing_invalid_direction() {
        let direction = "up";
        let result = FocusDirection::try_from(direction);
        assert!(result.is_err());
    }

    #[test]
    fn test_monitor_index_creation() {
        let monitor = 3;
        let monitor_index = MonitorIndex(monitor);
        assert_eq!(monitor_index.0, 3);
    }

    // Note: Testing the actual run() function and command execution would require
    // mocking the external tools and window management system, which is beyond
    // the scope of unit tests. Integration tests would be more appropriate for
    // testing the full command execution flow.
}
