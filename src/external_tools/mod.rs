mod utils;
pub mod wmctrl;
pub mod xdotool;
pub mod xrandr;

pub fn check_if_all_tools_installed() {
    wmctrl::check_if_installed();
    xdotool::check_if_installed();
    xrandr::check_if_installed();
}
