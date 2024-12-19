use anyhow::{Ok, Result};
use std::collections::HashMap;

use crate::external_tools::{wmctrl, xdotool, xrandr};
use crate::models::{FocusDirection, MonitorGrid, MonitorIndex, Window, WindowId, Workspace};

pub fn focus_by_direction(direction: FocusDirection) -> Result<()> {
    let workspace = xrandr::parse_workspace()?;
    let windows = get_current_workspace_windows(&workspace);
    let current_window_id = xdotool::get_current_focused_window_id();

    if let Some(window_to_focus) = find_closest_window(
        &current_window_id,
        &workspace.monitor_grid,
        &windows,
        &direction,
    )? {
        wmctrl::focus_window_by_id(&window_to_focus.id);
    }

    Ok(())
}

pub fn focus_by_monitor_index(index: MonitorIndex) -> Result<()> {
    let workspace = xrandr::parse_workspace()?;
    let windows = get_current_workspace_windows(&workspace);
    let windows_by_monitor_index = index_windows_by_monitor(&workspace.monitor_grid, &windows)?;

    if windows_by_monitor_index.contains_key(&index) {
        wmctrl::focus_window_by_id(&windows_by_monitor_index[&index][0].id);
    }

    Ok(())
}

fn get_current_workspace_windows(workspace: &Workspace) -> Vec<Window> {
    let mut current_workspace_windows = wmctrl::get_windows_config()
        .into_iter()
        .filter(|window| workspace.is_window_in_current_workspace(window))
        .collect::<Vec<Window>>();

    // Sort by the x-offset to make sure the Windows are in order from left to right.
    current_workspace_windows.sort_by(|a, b| a.x_offset.cmp(&b.x_offset));

    current_workspace_windows
}

fn index_windows_by_monitor<'a>(
    monitor_grid: &MonitorGrid,
    windows: &'a Vec<Window>,
) -> Result<HashMap<MonitorIndex, Vec<&'a Window>>> {
    let mut windows_by_monitor_index: HashMap<MonitorIndex, Vec<&Window>> = HashMap::new();

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
) -> Result<HashMap<WindowId, MonitorIndex>> {
    let mut monitors_by_window: HashMap<WindowId, MonitorIndex> = HashMap::new();

    for window in windows {
        monitors_by_window.insert(
            window.id.clone(),
            monitor_grid.determine_which_monitor_window_is_on(window)?,
        );
    }

    Ok(monitors_by_window)
}

fn get_current_monitor(
    current_window_id: &WindowId,
    monitors_by_window: &HashMap<WindowId, MonitorIndex>,
) -> MonitorIndex {
    monitors_by_window[current_window_id].clone()
}

fn find_closest_window(
    current_window_id: &WindowId,
    monitor_grid: &MonitorGrid,
    windows: &Vec<Window>,
    direction: &FocusDirection,
) -> Result<Option<Window>> {
    if windows.is_empty() {
        return Ok(None);
    }

    let windows_by_monitor = index_windows_by_monitor(monitor_grid, windows)?;
    let monitors_by_window = index_monitors_by_window(monitor_grid, windows)?;

    let current_monitor = get_current_monitor(current_window_id, &monitors_by_window);
    let current_monitor_windows = &windows_by_monitor[&current_monitor];

    if let Some(current_window_position) = current_monitor_windows
        .iter()
        .position(|w| w.id == *current_window_id)
    {
        if is_closest_window_not_on_current_monitor(
            direction,
            current_monitor_windows,
            current_window_position,
        ) {
            let mut next_monitor = monitor_grid.get_next_monitor(&current_monitor, direction);

            let mut optional_window =
                find_next_monitor_window(&windows_by_monitor, &next_monitor, direction);

            loop {
                match optional_window {
                    Some(window) => {
                        return Ok(Some(window.clone()));
                    }
                    None => {
                        next_monitor = monitor_grid.get_next_monitor(&next_monitor, direction);

                        optional_window =
                            find_next_monitor_window(&windows_by_monitor, &next_monitor, direction);
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

/// Given the windows of the current monitor, and the direction we want to focus to,
/// determines if we need to look at another monitor to find the correct window to focus to.
///
/// That is, if we're already at the leftmost/rightmost window, we need to look at the next
/// monitor to find the window to focus on.
fn is_closest_window_not_on_current_monitor(
    direction: &FocusDirection,
    current_monitor_windows: &[&Window],
    current_window_position: usize,
) -> bool {
    if current_monitor_windows.len() == 1 {
        true
    } else {
        match direction {
            FocusDirection::Left => current_window_position == 0,
            FocusDirection::Right => current_window_position == current_monitor_windows.len() - 1,
        }
    }
}

/// Used to "find the next monitor's window", using the focus direction as a signal for which side
/// of a monitor's windows to focus to.
///
/// That is, if switching to the left monitor, take the farthest right (i.e. last) window on the monitor.
/// If switching to the right monitor, take the farthest left (i.e. first) window on the monitor.
fn find_next_monitor_window<'a>(
    windows_by_monitor: &'a HashMap<MonitorIndex, Vec<&'a Window>>,
    monitor: &MonitorIndex,
    direction: &FocusDirection,
) -> Option<&'a Window> {
    if let Some(windows) = windows_by_monitor.get(monitor) {
        match direction {
            FocusDirection::Left => windows.last().map(|v| &**v),
            FocusDirection::Right => windows.first().map(|v| &**v),
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    mod is_closest_window_not_on_current_monitor {
        use super::*;

        fn create_mock_windows() -> Vec<Window> {
            let window1 = Window {
                id: WindowId(1),
                x_offset: 10,
                y_offset: 20,
                width: 30,
                height: 40,
                window_class: "class1".to_string(),
                title: "title1".to_string(),
            };

            let window2 = Window {
                id: WindowId(2),
                x_offset: 50,
                y_offset: 60,
                width: 70,
                height: 80,
                window_class: "class2".to_string(),
                title: "title2".to_string(),
            };

            vec![window1, window2]
        }

        #[test]
        fn test_left_true() {
            let windows = create_mock_windows();
            let window_refs: Vec<&Window> = windows.iter().collect();

            let result =
                is_closest_window_not_on_current_monitor(&FocusDirection::Left, &window_refs, 0);

            assert!(result);
        }

        #[test]
        fn test_left_false() {
            let windows = create_mock_windows();
            let window_refs: Vec<&Window> = windows.iter().collect();

            let result =
                is_closest_window_not_on_current_monitor(&FocusDirection::Left, &window_refs, 1);

            assert!(!result);
        }

        #[test]
        fn test_right_true() {
            let windows = create_mock_windows();
            let window_refs: Vec<&Window> = windows.iter().collect();

            let result =
                is_closest_window_not_on_current_monitor(&FocusDirection::Right, &window_refs, 1);

            assert!(result);
        }

        #[test]
        fn test_right_false() {
            let windows = create_mock_windows();
            let window_refs: Vec<&Window> = windows.iter().collect();

            let result =
                is_closest_window_not_on_current_monitor(&FocusDirection::Right, &window_refs, 0);

            assert!(!result);
        }

        #[test]
        fn test_one_window() {
            let mut windows = create_mock_windows();
            windows.truncate(1);

            let window_refs: Vec<&Window> = windows.iter().collect();

            assert!(is_closest_window_not_on_current_monitor(
                &FocusDirection::Left,
                &window_refs,
                1
            ));

            assert!(is_closest_window_not_on_current_monitor(
                &FocusDirection::Right,
                &window_refs,
                0
            ));
        }
    }

    mod find_next_monitor_window {
        use super::*;

        fn create_mock_windows() -> Vec<Window> {
            let window1 = Window {
                id: WindowId(1),
                x_offset: 10,
                y_offset: 20,
                width: 30,
                height: 40,
                window_class: "class1".to_string(),
                title: "title1".to_string(),
            };

            let window2 = Window {
                id: WindowId(2),
                x_offset: 50,
                y_offset: 60,
                width: 70,
                height: 80,
                window_class: "class2".to_string(),
                title: "title2".to_string(),
            };

            vec![window1, window2]
        }

        fn create_mock_index(
            windows: &[Window],
        ) -> (HashMap<MonitorIndex, Vec<&Window>>, MonitorIndex) {
            let mut windows_by_monitor: HashMap<MonitorIndex, Vec<&Window>> = HashMap::new();

            let monitor_index = MonitorIndex(0);
            windows_by_monitor.insert(monitor_index.clone(), vec![&windows[0], &windows[1]]);

            (windows_by_monitor, monitor_index)
        }

        #[test]
        fn test_left_monitor() {
            let windows = create_mock_windows();
            let (windows_by_monitor, monitor_index) = create_mock_index(&windows);

            let result = find_next_monitor_window(
                &windows_by_monitor,
                &monitor_index,
                &FocusDirection::Left,
            )
            .unwrap();

            assert_eq!(result.id, WindowId(2));
        }

        #[test]
        fn test_right_monitor() {
            let windows = create_mock_windows();
            let (windows_by_monitor, monitor_index) = create_mock_index(&windows);

            let result = find_next_monitor_window(
                &windows_by_monitor,
                &monitor_index,
                &FocusDirection::Right,
            )
            .unwrap();

            assert_eq!(result.id, WindowId(1));
        }

        #[test]
        fn test_one_window_monitor() {
            let windows = create_mock_windows();
            let (mut windows_by_monitor, monitor_index) = create_mock_index(&windows);

            windows_by_monitor
                .get_mut(&monitor_index)
                .unwrap()
                .truncate(1);

            let result1 = find_next_monitor_window(
                &windows_by_monitor,
                &monitor_index,
                &FocusDirection::Left,
            )
            .unwrap();

            let result2 = find_next_monitor_window(
                &windows_by_monitor,
                &monitor_index,
                &FocusDirection::Right,
            )
            .unwrap();

            assert_eq!(result1.id, WindowId(1));
            assert_eq!(result2.id, WindowId(1));
        }

        #[test]
        fn test_no_windows() {
            let windows_by_monitor = HashMap::new();
            let monitor_index = MonitorIndex(0);

            let result = find_next_monitor_window(
                &windows_by_monitor,
                &monitor_index,
                &FocusDirection::Right,
            );

            assert!(result.is_none());
        }
    }
}
