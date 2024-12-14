use anyhow::Result;

/// The height of the window decoration that is constant in Ubuntu.
pub const WINDOW_DECORATION: i32 = 24;

/// # Window Model
///
/// Models the attributes of a single window (on a monitor, in a workspace, in the workspace grid).
/// Specifically, it cares about things like where the window is positioned relative to the current
/// workspace (i.e. x and y offset) as well as the ID/title of the window.
///
/// ## Fields
///
/// - id: An integer representation of the window's ID (normally in hex).
/// - x_offset and y_offset:
///     - x and y offset are how windows (specifically, their top-left corner, not including window decoration)
///         are positioned relative to the current workspace. Some examples (given a triple 1080p monitor setup):
///     - An x,y offset of 0,0 would put the window on the left-most monitor.
///     - An x,y offset of 0,24 also puts the window on the left-most monitor,
///         but the y-offset has accounted for window decoration (this is what's most commonly seen).
///     - An x,y offset of 1920,24 puts the window in the center monitor, because it is positioned 1920 pixels
///         from the left-most edge of the workspace.
/// - height: The height of the window (in pixels).
/// - width: The width of the window (in pixels).
/// - window_class: The class of the window (e.g. "google-chrome.Google-chrome")
/// - title: The title of the window.
#[derive(Debug)]
pub struct Window {
    id: u64,
    x_offset: i32,
    y_offset: i32,
    width: i32,
    height: i32,
    window_class: String,
    title: String,
}

impl Window {
    pub fn new(
        id: u64,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        window_class: String,
        title: String,
    ) -> Self {
        Self {
            id,
            x_offset,
            y_offset,
            width,
            height,
            window_class,
            title,
        }
    }

    /// Processes the raw string representation of the window config into
    /// all of the attributes needed for the Window instance.
    ///
    /// Example: "0x05000006  0 1920 24   1920 1056 gnome-terminal-server.Gnome-terminal  devin-Desktop Terminal"
    ///
    /// Column 0 is the window ID (0x05000006)
    /// Column 1 is the 'desktop index' (almost always 0 for our uses in Unity; can just ignore)
    /// Column 2 is the x-offset (1920)
    /// Column 3 is the y-offset (24)
    /// Column 4 is the window width (1920)
    /// Column 5 is the window height (1056)
    /// Column 6 is the WM_CLASS property from the '-x' option (gnome-terminal-server.Gnome-terminal)
    /// Column 7 is the hostname (devin-Desktop)
    /// Column 8+ is the title of the window (Terminal)
    pub fn from_raw_config(raw_config: &str) -> Result<Self> {
        let split_config: Vec<&str> = raw_config.split_whitespace().collect();

        println!("{:?}", split_config);

        let id = Window::parse_hex_string(split_config[0])?;
        let x_offset = split_config[2].parse::<i32>()?;
        let y_offset = split_config[3].parse::<i32>()?;
        let width = split_config[4].parse::<i32>()?;
        let height = split_config[5].parse::<i32>()?;
        let window_class = split_config[6].to_string();
        let title: String = split_config[8..].join(" "); // Skip column 7 since we don't care about the hostname.

        Ok(Self {
            id,
            x_offset,
            y_offset,
            height,
            width,
            window_class,
            title,
        })
    }

    fn parse_hex_string(hex_string: &str) -> Result<u64> {
        Ok(u64::from_str_radix(
            hex_string.trim_start_matches("0x"),
            16,
        )?)
    }
}

impl std::fmt::Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ID: {}\nX Offset: {}\nY Offset: {}\nDimensions: {}x{}\nClass: {}\nTitle: {}",
            self.id,
            self.x_offset,
            self.y_offset,
            self.width,
            self.height,
            self.window_class,
            self.title
        )
    }
}

// Test cases for Window constructor (new function)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_creation() {
        let window = Window::new(
            0x05000006,
            1920,
            24,
            1920,
            1056,
            "gnome-terminal-server.Gnome-terminal".to_string(),
            "Terminal".to_string(),
        );

        assert_eq!(window.id, 83886086);
        assert_eq!(window.x_offset, 1920);
        assert_eq!(window.y_offset, 24);
        assert_eq!(window.width, 1920);
        assert_eq!(window.height, 1056);
        assert_eq!(window.window_class, "gnome-terminal-server.Gnome-terminal");
        assert_eq!(window.title, "Terminal");
    }

    // Test case for from_raw_config function
    #[test]
    fn test_from_raw_config() {
        let raw_config = "0x05000006  0 1920 24   1920 1056 gnome-terminal-server.Gnome-terminal  devin-Desktop Terminal";

        match Window::from_raw_config(raw_config) {
            Ok(window) => {
                assert_eq!(window.id, 83886086);
                assert_eq!(window.x_offset, 1920);
                assert_eq!(window.y_offset, 24);
                assert_eq!(window.width, 1920);
                assert_eq!(window.height, 1056);
                assert_eq!(window.window_class, "gnome-terminal-server.Gnome-terminal");
                assert_eq!(window.title, "Terminal".to_string());
            }
            Err(e) => panic!("Failed to parse raw config: {}", e),
        }
    }

    // Test case for incorrect raw config format
    #[test]
    fn test_invalid_raw_config() {
        let raw_config = "Invalid Config";

        if Window::from_raw_config(raw_config).is_ok() {
            panic!("Expected error for invalid config")
        }
    }
}

// Test cases for Display implementation
#[cfg(test)]
mod display_tests {
    use super::*;

    #[test]
    fn test_display() {
        let window = Window::new(
            0x05000006,
            1920,
            24,
            1920,
            1056,
            "gnome-terminal-server.Gnome-terminal".to_string(),
            "Terminal".to_string(),
        );

        let expected_output = "ID: 83886086\nX Offset: 1920\nY Offset: 24\nDimensions: 1920x1056\nClass: gnome-terminal-server.Gnome-terminal\nTitle: Terminal";
        assert_eq!(format!("{}", window), expected_output);
    }
}
