use std::collections::HashMap;

use crate::external_tools::wmctrl;
use crate::models::{Window, WINDOW_DECORATION};

pub fn focus_by_monitor_index(index: usize) {
    let windows_by_monitor_index = index_windows_by_monitor();

    if windows_by_monitor_index.len() > index {
        wmctrl::focus_window_by_id(windows_by_monitor_index[&index][0].id);
    }
}

fn get_current_workspace_windows() -> Vec<Window> {
    let (workspace_grid, _) = wmctrl::get_workspace_config();

    let mut current_workspace_windows = wmctrl::get_windows_config()
        .into_iter()
        .filter(|window| workspace_grid.window_in_current_workspace(window))
        .collect::<Vec<Window>>();

    // Sort by the x-offset to make sure the Windows are in order from left to right.
    current_workspace_windows.sort_by(|a, b| a.x_offset.cmp(&b.x_offset));

    current_workspace_windows
}

fn index_windows_by_monitor() -> HashMap<usize, Vec<Window>> {
    let windows = get_current_workspace_windows();

    let mut windows_by_monitor_index: HashMap<usize, Vec<Window>> = HashMap::new();

    for window in windows.into_iter() {
        let monitor_index = determine_which_monitor_window_is_on(&window);

        windows_by_monitor_index
            .entry(monitor_index)
            .or_default()
            .push(window);
    }

    windows_by_monitor_index
}

// TODO: Rewrite this to use the monitor arrangement.
fn determine_which_monitor_window_is_on(window: &Window) -> usize {
    if window.x_offset < 1920 {
        ((window.y_offset - WINDOW_DECORATION) / 1080) as usize
    } else if window.x_offset >= 1920 && window.x_offset < (1920 + 3440) {
        2
    } else {
        3
    }
}
