use super::utils::{call_command, get_command_output};
use crate::models::{Window, WorkspaceGrid, WorkspacePosition};

pub fn get_workspace_config() -> (WorkspaceGrid, WorkspacePosition) {
    let workspace_config = get_command_output(&["wmctrl", "-d"]);
    parse_workspace_config(&workspace_config)
}

pub fn get_windows_config() -> Vec<Window> {
    let windows_config = get_command_output(&["wmctrl", "-l", "-G", "-x"]);
    parse_windows_config(&windows_config)
}

pub fn focus_window_by_id(window_id: i32) {
    call_command(&["wmctrl", "-i", "-a", &window_id.to_string()]);
}

fn parse_workspace_config(system_config: &str) -> (WorkspaceGrid, WorkspacePosition) {
    let first_splits = system_config
        .split("DG:")
        .nth(1)
        .unwrap()
        .split("VP:")
        .collect::<Vec<&str>>();

    let workspace_grid = WorkspaceGrid::from_string_dimensions(first_splits[0]);

    let current_workspace_position = WorkspacePosition::from_string_position(
        first_splits[1].split("WA:").next().unwrap().trim(),
    );

    (workspace_grid, current_workspace_position)
}

fn parse_windows_config(windows_config: &str) -> Vec<Window> {
    let split_windows_config: Vec<&str> = windows_config.split("\n").collect();
    let mut windows = Vec::new();

    for window_config in split_windows_config {
        if !window_config.is_empty() {
            let window = Window::from_raw_config(window_config).unwrap();

            if window.window_class != "N/A"
                && window.window_class != "nemo-desktop.Nemo-desktop"
                && window.y_offset > 0
            {
                windows.push(window);
            }
        }
    }

    windows
}
