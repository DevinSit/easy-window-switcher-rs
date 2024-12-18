use anyhow::{Ok, Result};
use std::collections::HashMap;

use crate::external_tools::{wmctrl, xdotool, xrandr};
use crate::models::{FocusDirection, MonitorGrid, Window, WindowId};

pub fn focus_by_direction(direction: FocusDirection) -> Result<()> {
    let monitor_grid = xrandr::get_monitor_grid()?;
    let windows = get_current_workspace_windows(&monitor_grid);

    if let Some(window_to_focus) = get_closest_window(&monitor_grid, &windows, &direction)? {
        wmctrl::focus_window_by_id(&window_to_focus.id);
    }

    Ok(())
}

pub fn focus_by_monitor_index(index: usize) -> Result<()> {
    let monitor_grid = xrandr::get_monitor_grid()?;
    let windows = get_current_workspace_windows(&monitor_grid);
    let windows_by_monitor_index = index_windows_by_monitor(&monitor_grid, &windows)?;

    if windows_by_monitor_index.contains_key(&index) {
        wmctrl::focus_window_by_id(&windows_by_monitor_index[&index][0].id);
    }

    Ok(())
}

fn get_current_workspace_windows(monitor_grid: &MonitorGrid) -> Vec<Window> {
    let mut current_workspace_windows = wmctrl::get_windows_config()
        .into_iter()
        .filter(|window| monitor_grid.is_window_in_current_workspace(window))
        .collect::<Vec<Window>>();

    // Sort by the x-offset to make sure the Windows are in order from left to right.
    current_workspace_windows.sort_by(|a, b| a.x_offset.cmp(&b.x_offset));

    current_workspace_windows
}

fn index_windows_by_monitor<'a>(
    monitor_grid: &MonitorGrid,
    windows: &'a Vec<Window>,
) -> Result<HashMap<usize, Vec<&'a Window>>> {
    let mut windows_by_monitor_index: HashMap<usize, Vec<&Window>> = HashMap::new();

    for window in windows {
        let monitor_index = monitor_grid.determine_which_monitor_window_is_on(window)?;

        windows_by_monitor_index
            .entry(monitor_index)
            .or_default()
            .push(window);
    }

    Ok(windows_by_monitor_index)
}

fn index_monitors_by_window(
    monitor_grid: &MonitorGrid,
    windows: &Vec<Window>,
) -> Result<HashMap<WindowId, usize>> {
    let mut monitors_by_window: HashMap<WindowId, usize> = HashMap::new();

    for window in windows {
        monitors_by_window.insert(
            window.id.clone(),
            monitor_grid.determine_which_monitor_window_is_on(window)?,
        );
    }

    Ok(monitors_by_window)
}

fn get_current_monitor(monitors_by_window: &HashMap<WindowId, usize>) -> usize {
    monitors_by_window[&WindowId(xdotool::get_current_focused_window_id())]
}

fn get_closest_window(
    monitor_grid: &MonitorGrid,
    windows: &Vec<Window>,
    direction: &FocusDirection,
) -> Result<Option<Window>> {
    if windows.is_empty() {
        return Ok(None);
    }

    let windows_by_monitor = index_windows_by_monitor(monitor_grid, windows)?;
    let monitors_by_window = index_monitors_by_window(monitor_grid, windows)?;

    let current_monitor = get_current_monitor(&monitors_by_window);
    let current_monitor_windows = &windows_by_monitor[&current_monitor];

    if let Some(current_window_position) = current_monitor_windows
        .iter()
        .position(|w| w.id == WindowId(xdotool::get_current_focused_window_id()))
    {
        if is_closest_window_not_on_current_monitor(
            direction,
            current_monitor_windows,
            current_window_position,
        ) {
            let mut next_monitor = monitor_grid.get_next_monitor(current_monitor, direction);

            let mut optional_window =
                get_window_from_monitor(&windows_by_monitor, next_monitor, direction);

            loop {
                match optional_window {
                    Some(window) => {
                        return Ok(Some(window.clone()));
                    }
                    None => {
                        next_monitor = monitor_grid.get_next_monitor(next_monitor, direction);

                        optional_window =
                            get_window_from_monitor(&windows_by_monitor, next_monitor, direction);
                    }
                }
            }
        } else {
            let position = (current_window_position as i32 + direction.to_int()) as usize;
            Ok(Some(current_monitor_windows[position].clone()))
        }
    } else {
        Err(anyhow::anyhow!(
            "Invariant violated: current focused window not found on current monitor"
        ))
    }
}

fn is_closest_window_not_on_current_monitor(
    direction: &FocusDirection,
    current_monitor_windows: &[&Window],
    current_window_position: usize,
) -> bool {
    match direction {
        FocusDirection::Left => current_monitor_windows.len() == 1 || current_window_position == 0,
        FocusDirection::Right => {
            current_monitor_windows.len() == 1
                || current_window_position == current_monitor_windows.len() - 1
        }
    }
}

fn get_window_from_monitor<'a>(
    windows_by_monitor: &'a HashMap<usize, Vec<&'a Window>>,
    monitor: usize,
    direction: &FocusDirection,
) -> Option<&'a Window> {
    if let Some(windows) = windows_by_monitor.get(&monitor) {
        match direction {
            FocusDirection::Left => windows.last().map(|v| &**v),
            FocusDirection::Right => windows.first().map(|v| &**v),
        }
    } else {
        None
    }
}
