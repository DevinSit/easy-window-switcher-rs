use super::utils::get_command_output;

pub fn get_current_focused_window_id() -> i32 {
    let output = get_command_output(&["xdotool", "getwindowfocus"])
        .trim()
        .to_owned();

    output.parse::<i32>().unwrap()
}
