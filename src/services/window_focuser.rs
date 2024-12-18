use anyhow::Result;
use std::collections::HashMap;

use crate::external_tools::{wmctrl, xdotool};
use crate::models::{Window, WorkspaceGrid};

// TODO: Derive this from monitor arrangement.
const NUMBER_OF_MONITORS: i32 = 4;

pub fn focus_by_direction(direction: &str) -> Result<()> {
    let workspace_grid = wmctrl::get_workspace_config();
    let windows = get_current_workspace_windows(&workspace_grid);

    if let Some(window_to_focus) = get_closest_window(&workspace_grid, &windows, direction)? {
        wmctrl::focus_window_by_id(window_to_focus.id);
    }

    Ok(())
}

pub fn focus_by_monitor_index(index: usize) -> Result<()> {
    let workspace_grid = wmctrl::get_workspace_config();
    let windows = get_current_workspace_windows(&workspace_grid);
    let windows_by_monitor_index = index_windows_by_monitor(&workspace_grid, &windows)?;

    if windows_by_monitor_index.contains_key(&index) {
        wmctrl::focus_window_by_id(windows_by_monitor_index[&index][0].id);
    }

    Ok(())
}

fn get_current_workspace_windows(workspace_grid: &WorkspaceGrid) -> Vec<Window> {
    let mut current_workspace_windows = wmctrl::get_windows_config()
        .into_iter()
        .filter(|window| workspace_grid.is_window_in_current_workspace(window))
        .collect::<Vec<Window>>();

    // Sort by the x-offset to make sure the Windows are in order from left to right.
    current_workspace_windows.sort_by(|a, b| a.x_offset.cmp(&b.x_offset));

    current_workspace_windows
}

fn index_windows_by_monitor<'a>(
    grid: &WorkspaceGrid,
    windows: &'a Vec<Window>,
) -> Result<HashMap<usize, Vec<&'a Window>>> {
    let mut windows_by_monitor_index: HashMap<usize, Vec<&Window>> = HashMap::new();

    for window in windows {
        let monitor_index = grid.determine_which_monitor_window_is_on(window)?;

        windows_by_monitor_index
            .entry(monitor_index)
            .or_default()
            .push(window);
    }

    Ok(windows_by_monitor_index)
}

fn index_monitors_by_window(
    grid: &WorkspaceGrid,
    windows: &Vec<Window>,
) -> Result<HashMap<usize, usize>> {
    let mut monitors_by_window: HashMap<usize, usize> = HashMap::new();

    for window in windows {
        monitors_by_window.insert(
            window.id,
            grid.determine_which_monitor_window_is_on(window)?,
        );
    }

    Ok(monitors_by_window)
}

fn get_current_monitor(monitors_by_window: HashMap<usize, usize>) -> usize {
    let current_focused_window_id = xdotool::get_current_focused_window_id();
    monitors_by_window[&current_focused_window_id]
}

fn get_closest_window(
    workspace_grid: &WorkspaceGrid,
    windows: &Vec<Window>,
    direction: &str,
) -> Result<Option<Window>> {
    let windows_by_monitor = index_windows_by_monitor(workspace_grid, windows)?;
    let monitors_by_window = index_monitors_by_window(workspace_grid, windows)?;

    let current_monitor = get_current_monitor(monitors_by_window);
    let current_monitor_windows = &windows_by_monitor[&current_monitor];

    let current_window_position = current_monitor_windows
        .iter()
        .position(|w| w.id == xdotool::get_current_focused_window_id())
        .unwrap();

    if windows.is_empty() {
        Ok(None)
    } else {
        match direction {
            "left" => {
                if is_leftmost_window_on_current_monitor(
                    current_monitor_windows,
                    current_window_position,
                ) {
                    let mut left_monitor = next_monitor(current_monitor.try_into().unwrap(), -1);
                    let mut optional_window =
                        get_window_from_monitor(&windows_by_monitor, left_monitor, -1);

                    loop {
                        match optional_window {
                            Some(window) => {
                                return Ok(Some(window.clone()));
                            }
                            None => {
                                left_monitor = next_monitor(left_monitor, -1);

                                optional_window =
                                    get_window_from_monitor(&windows_by_monitor, left_monitor, -1);
                            }
                        }
                    }
                } else {
                    Ok(Some(
                        current_monitor_windows[current_window_position - 1].clone(),
                    ))
                }
            }
            "right" => {
                if is_rightmost_window_on_current_monitor(
                    current_monitor_windows,
                    current_window_position,
                ) {
                    let mut left_monitor = next_monitor(current_monitor.try_into().unwrap(), 1);
                    let mut optional_window =
                        get_window_from_monitor(&windows_by_monitor, left_monitor, 0);

                    loop {
                        match optional_window {
                            Some(window) => {
                                return Ok(Some(window.clone()));
                            }
                            None => {
                                left_monitor = next_monitor(left_monitor, 1);

                                optional_window =
                                    get_window_from_monitor(&windows_by_monitor, left_monitor, 0);
                            }
                        }
                    }
                } else {
                    Ok(Some(
                        current_monitor_windows[current_window_position + 1].clone(),
                    ))
                }
            }
            _ => Err(anyhow::anyhow!("Invalid direction: {direction}")),
        }
    }
}

fn is_leftmost_window_on_current_monitor(
    current_monitor_windows: &[&Window],
    current_window_position: usize,
) -> bool {
    current_monitor_windows.len() == 1 || current_window_position == 0
}

fn is_rightmost_window_on_current_monitor(
    current_monitor_windows: &[&Window],
    current_window_position: usize,
) -> bool {
    current_monitor_windows.len() == 1
        || current_window_position == current_monitor_windows.len() - 1
}

fn get_window_from_monitor<'a>(
    windows_by_monitor: &'a HashMap<usize, Vec<&'a Window>>,
    monitor: i32,
    index: i32,
) -> Option<&'a Window> {
    if let Some(windows) = windows_by_monitor.get(&(monitor as usize)) {
        if index < 0 {
            windows.last().map(|v| &**v)
        } else {
            Some(windows[index as usize])
        }
    } else {
        None
    }
}

fn next_monitor(current_monitor: i32, direction: i32) -> i32 {
    // Need to this song and dance to get the modulo behavior we want.
    // Otherwise, we can get a negative remainder.
    //
    // Ref: https://stackoverflow.com/q/31210357
    (((current_monitor + direction) % NUMBER_OF_MONITORS) + NUMBER_OF_MONITORS) % NUMBER_OF_MONITORS
}
