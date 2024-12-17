use super::utils::{call_command, get_command_output};
use crate::models::{Window, WorkspaceGrid};

pub fn get_workspace_config() -> WorkspaceGrid {
    let workspace_config = get_command_output(&["wmctrl", "-d"]);
    parse_workspace_config(&workspace_config)
}

pub fn get_windows_config() -> Vec<Window> {
    let windows_config = get_command_output(&["wmctrl", "-l", "-G", "-x"]);
    parse_windows_config(&windows_config)
}

pub fn focus_window_by_id(window_id: usize) {
    call_command(&["wmctrl", "-i", "-a", &window_id.to_string()]);
}

fn parse_workspace_config(system_config: &str) -> WorkspaceGrid {
    let first_splits = system_config
        .split("DG:")
        .nth(1)
        .unwrap()
        .split("VP:")
        .collect::<Vec<&str>>();

    WorkspaceGrid::from_string_dimensions(first_splits[0])
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_workspace_config() {
        let grid = get_workspace_config();

        assert!(grid.width > 0);
        assert!(grid.height > 0);
    }

    #[test]
    fn test_get_windows_config() {
        let windows = get_windows_config();

        assert!(!windows.is_empty());
    }

    #[test]
    fn test_parse_workspace_config() {
        let workspace_config = "0  * DG: 20400x7680  VP: 6800,0  WA: 0,24 6800x2536  N/A";
        let grid = parse_workspace_config(workspace_config);

        assert_eq!(grid.width, 20400);
        assert_eq!(grid.height, 7680);
    }

    #[test]
    fn test_parse_windows_config() {
        let windows_config = [
            // Should get excluded cause of `N/A` window class.
            "0x0340000b  0 -159 -1156 59   1056 N/A                   devin-5900x unity-launcher",
            // Should get excluded cause of `nemo-desktop.Nemo-desktop` window class.
            "0x03800003 -1 0    1080 1920 1080 nemo-desktop.Nemo-desktop  devin-5900x Desktop",
            // Should get excluded cause of 0 `y_offset`.
            // (Note: not a real window config; added for testing purposes)
            "0x04a00006  0 1920 0  3440 1416 code.Code             devin-5900x wmctrl.rs - easy-window-switcher-rs - Visual Studio Code",
            // This is the only real window that should be included.
            "0x04a00006  0 1920 564  3440 1416 code.Code             devin-5900x wmctrl.rs - easy-window-switcher-rs - Visual Studio Code"
        ].join("\n");

        let windows = parse_windows_config(&windows_config);

        assert_eq!(windows.len(), 1);

        if let Some(window) = windows.iter().find(|w| w.window_class == "code.Code") {
            assert_eq!(window.id, 77594630);
            assert_eq!(window.x_offset, 1920);
            assert_eq!(window.y_offset, 564);
            assert_eq!(window.width, 3440);
            assert_eq!(window.height, 1416);

            assert_eq!(
                window.title,
                "wmctrl.rs - easy-window-switcher-rs - Visual Studio Code"
            );
        } else {
            panic!("Failed to parse window correctly");
        }
    }
}
