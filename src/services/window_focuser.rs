use anyhow::Result;
use std::collections::HashMap;

use crate::external_tools::{wmctrl, xdotool, xrandr};
use crate::models::{FocusDirection, MonitorGrid, Window};

pub fn focus_by_direction(direction: FocusDirection) -> Result<()> {
    let monitor_grid = xrandr::get_monitor_grid()?;
    let windows = get_current_workspace_windows(&monitor_grid);

    if let Some(window_to_focus) = get_closest_window(&monitor_grid, &windows, &direction)? {
        wmctrl::focus_window_by_id(window_to_focus.id);
    }

    Ok(())
}

pub fn focus_by_monitor_index(index: usize) -> Result<()> {
    let monitor_grid = xrandr::get_monitor_grid()?;
    let windows = get_current_workspace_windows(&monitor_grid);
    let windows_by_monitor_index = index_windows_by_monitor(&monitor_grid, &windows)?;

    if windows_by_monitor_index.contains_key(&index) {
        wmctrl::focus_window_by_id(windows_by_monitor_index[&index][0].id);
    }

    Ok(())
}

fn get_current_workspace_windows(grid: &MonitorGrid) -> Vec<Window> {
    let mut current_workspace_windows = wmctrl::get_windows_config()
        .into_iter()
        .filter(|window| grid.is_window_in_current_workspace(window))
        .collect::<Vec<Window>>();

    // Sort by the x-offset to make sure the Windows are in order from left to right.
    current_workspace_windows.sort_by(|a, b| a.x_offset.cmp(&b.x_offset));

    current_workspace_windows
}

fn index_windows_by_monitor<'a>(
    grid: &MonitorGrid,
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
    grid: &MonitorGrid,
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
    grid: &MonitorGrid,
    windows: &Vec<Window>,
    direction: &FocusDirection,
) -> Result<Option<Window>> {
    let windows_by_monitor = index_windows_by_monitor(grid, windows)?;
    let monitors_by_window = index_monitors_by_window(grid, windows)?;

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
            FocusDirection::Left => {
                if is_leftmost_window_on_current_monitor(
                    current_monitor_windows,
                    current_window_position,
                ) {
                    let mut left_monitor =
                        grid.get_next_monitor(current_monitor, direction.clone());

                    let mut optional_window =
                        get_window_from_monitor(&windows_by_monitor, left_monitor, -1);

                    loop {
                        match optional_window {
                            Some(window) => {
                                return Ok(Some(window.clone()));
                            }
                            None => {
                                left_monitor =
                                    grid.get_next_monitor(left_monitor, direction.clone());

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
            FocusDirection::Right => {
                if is_rightmost_window_on_current_monitor(
                    current_monitor_windows,
                    current_window_position,
                ) {
                    let mut left_monitor =
                        grid.get_next_monitor(current_monitor, direction.clone());

                    let mut optional_window =
                        get_window_from_monitor(&windows_by_monitor, left_monitor, 0);

                    loop {
                        match optional_window {
                            Some(window) => {
                                return Ok(Some(window.clone()));
                            }
                            None => {
                                left_monitor =
                                    grid.get_next_monitor(left_monitor, direction.clone());

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
    monitor: usize,
    index: i32,
) -> Option<&'a Window> {
    if let Some(windows) = windows_by_monitor.get(&monitor) {
        if index < 0 {
            // This is a direct conversion of Python's "-1 index is the last element" idiom.
            windows.last().map(|v| &**v)
        } else {
            Some(windows[index as usize])
        }
    } else {
        None
    }
}
