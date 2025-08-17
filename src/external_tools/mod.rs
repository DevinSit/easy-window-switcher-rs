mod utils;
pub mod wmctrl;
pub mod xdotool;
pub mod xrandr;

pub fn check_if_all_tools_installed() {
    wmctrl::check_if_installed();
    xdotool::check_if_installed();
    xrandr::check_if_installed();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_if_all_tools_installed() {
        // This test just ensures the function runs without panicking
        // In a real system with the tools installed, it should complete successfully
        // On systems without the tools, it would exit(1), but we can't easily test that
        // in unit tests without mocking
        check_if_all_tools_installed();
    }
}
