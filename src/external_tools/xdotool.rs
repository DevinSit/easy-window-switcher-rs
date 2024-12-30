use super::utils::{get_command_output, is_tool_installed};
use crate::models::WindowId;

pub fn check_if_installed() {
    if !is_tool_installed("xdotool") {
        eprintln!("Error: xdotool is not installed; please install it first through your e.g. package manager");
        std::process::exit(1);
    }
}

pub fn get_current_focused_window_id() -> WindowId {
    let output = get_command_output(&["xdotool", "getwindowfocus"])
        .trim()
        .to_owned();

    WindowId(output.parse::<usize>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_focused_window_id() {
        let id = get_current_focused_window_id();

        assert!(id.0 > 0);
    }
}
