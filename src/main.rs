use easy_window_switcher_rs::external_tools::xdotool;

fn main() {
    println!("{:?}", xdotool::get_current_focused_window_id());
}
