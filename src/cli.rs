use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::models::FocusDirection;
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
        Commands::Monitor { monitor } => window_focuser::focus_by_monitor_index(monitor),
    }
}
