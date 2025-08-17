use anyhow::Result;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct WindowId(pub usize);

impl std::fmt::Display for WindowId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The height of the window decoration that is constant in Ubuntu.
pub const WINDOW_DECORATION: i32 = 24;

/// Models the attributes of a single window (on a Monitor).
/// Specifically, it cares about things like where the window is positioned relative to the current
/// Workspace (i.e. x and y offset) as well as the ID/title of the window.
///
/// Fields:
///
/// - id: An integer representation of the window's ID (normally in hex).
/// - x_offset and y_offset:
///     x and y offset are how windows (specifically, their top-left corner, not including window decoration)
///     are positioned relative to the current workspace. Some examples (given a triple 1080p monitor setup):
///         - An x,y offset of 0,0 would put the window on the left-most monitor.
///         - An x,y offset of 0,24 also puts the window on the left-most monitor,
///             but the y-offset has accounted for window decoration (this is what's most commonly seen).
///         - An x,y offset of 1920,24 puts the window in the center monitor, because it is positioned 1920 pixels
///             from the left-most edge of the workspace.
/// - height: The height of the window (in pixels).
/// - width: The width of the window (in pixels).
/// - window_class: The class of the window (e.g. "google-chrome.Google-chrome")
/// - title: The title of the window.
#[derive(Clone, Debug)]
pub struct Window {
    pub id: WindowId,
    pub x_offset: i32,
    pub y_offset: i32,
    pub width: i32,
    pub height: i32,
    pub window_class: String,
    pub title: String,
}

impl Window {
    pub fn new(
        id: WindowId,
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

        let id = Self::parse_id(split_config[0])?;
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

    fn parse_id(hex_string: &str) -> Result<WindowId> {
        Ok(WindowId(usize::from_str_radix(
            hex_string.trim_start_matches("0x"),
            16,
        )?))
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

    mod window_id {
        use super::*;

        #[test]
        fn test_window_id_creation() {
            let id = WindowId(123456);
            assert_eq!(id.0, 123456);
        }

        #[test]
        fn test_window_id_display() {
            let id = WindowId(0x05000006);
            assert_eq!(format!("{}", id), "83886086");
        }

        #[test]
        fn test_window_id_equality() {
            let id1 = WindowId(100);
            let id2 = WindowId(100);
            let id3 = WindowId(200);

            assert_eq!(id1, id2);
            assert_ne!(id1, id3);
        }

        #[test]
        fn test_window_id_clone() {
            let id1 = WindowId(42);
            let id2 = id1.clone();
            assert_eq!(id1.0, id2.0);
        }
    }

    mod constants {
        use super::*;

        #[test]
        fn test_window_decoration_constant() {
            assert_eq!(WINDOW_DECORATION, 24);
        }
    }

    mod parse_id {
        use super::*;

        #[test]
        fn test_parse_valid_hex() {
            let result = Window::parse_id("0x05000006").unwrap();
            assert_eq!(result, WindowId(83886086));
        }

        #[test]
        fn test_parse_valid_hex_lowercase() {
            let result = Window::parse_id("0x05a0000b").unwrap();
            assert_eq!(result, WindowId(94371851));
        }

        #[test]
        fn test_parse_valid_hex_uppercase() {
            let result = Window::parse_id("0x05A0000B").unwrap();
            assert_eq!(result, WindowId(94371851));
        }

        #[test]
        fn test_parse_invalid_hex_format() {
            // This actually works because parse_id just strips "0x" if present
            let result = Window::parse_id("05000006").unwrap();
            assert_eq!(result, WindowId(83886086));
        }

        #[test]
        fn test_parse_missing_hex_prefix_still_works() {
            let result = Window::parse_id("ABC").unwrap();
            assert_eq!(result, WindowId(2748)); // ABC in hex = 2748 in decimal
        }

        #[test]
        fn test_parse_invalid_hex_characters() {
            let result = Window::parse_id("0xGGGGGGGG");
            assert!(result.is_err());
        }

        #[test]
        fn test_parse_empty_string() {
            let result = Window::parse_id("");
            assert!(result.is_err());
        }

        #[test]
        fn test_parse_malformed_prefix() {
            let result = Window::parse_id("0X05000006");
            assert!(result.is_err());
        }
    }

    mod additional_from_raw_config_tests {
        use super::*;

        #[test]
        fn test_from_raw_config_with_long_title() {
            let raw_config = "0x05000006  0 1920 24   1920 1056 gnome-terminal-server.Gnome-terminal  devin-Desktop This is a very long window title with many words";
            let window = Window::from_raw_config(raw_config).unwrap();
            assert_eq!(
                window.title,
                "This is a very long window title with many words"
            );
        }

        #[test]
        fn test_from_raw_config_with_single_word_title() {
            let raw_config = "0x05000006  0 1920 24   1920 1056 gnome-terminal-server.Gnome-terminal  devin-Desktop Terminal";
            let window = Window::from_raw_config(raw_config).unwrap();
            assert_eq!(window.title, "Terminal");
        }

        #[test]
        fn test_from_raw_config_with_empty_title() {
            let raw_config = "0x05000006  0 1920 24   1920 1056 gnome-terminal-server.Gnome-terminal  devin-Desktop";
            let window = Window::from_raw_config(raw_config).unwrap();
            assert_eq!(window.title, "");
        }

        #[test]
        #[should_panic(expected = "index out of bounds")]
        fn test_from_raw_config_too_few_parts() {
            let raw_config = "0x05000006  0 1920";
            let _result = Window::from_raw_config(raw_config);
            // This will panic when trying to access split_config[3] because there are only 3 parts (indices 0,1,2)
            // The function doesn't validate the input length before accessing array elements
        }

        #[test]
        fn test_from_raw_config_invalid_id() {
            let raw_config = "invalid_id  0 1920 24   1920 1056 gnome-terminal-server.Gnome-terminal  devin-Desktop Terminal";
            let result = Window::from_raw_config(raw_config);
            assert!(result.is_err());
        }

        #[test]
        fn test_from_raw_config_invalid_numbers() {
            let raw_config = "0x05000006  0 abc 24   1920 1056 gnome-terminal-server.Gnome-terminal  devin-Desktop Terminal";
            let result = Window::from_raw_config(raw_config);
            assert!(result.is_err());
        }

        #[test]
        fn test_from_raw_config_negative_values() {
            let raw_config = "0x05000006  0 -100 -50   1920 1056 gnome-terminal-server.Gnome-terminal  devin-Desktop Terminal";
            let window = Window::from_raw_config(raw_config).unwrap();
            assert_eq!(window.x_offset, -100);
            assert_eq!(window.y_offset, -50);
        }

        #[test]
        fn test_from_raw_config_zero_dimensions() {
            let raw_config = "0x05000006  0 0 0   0 0 gnome-terminal-server.Gnome-terminal  devin-Desktop Terminal";
            let window = Window::from_raw_config(raw_config).unwrap();
            assert_eq!(window.width, 0);
            assert_eq!(window.height, 0);
        }
    }

    #[test]
    fn test_window_creation() {
        let window = Window::new(
            WindowId(0x05000006),
            1920,
            24,
            1920,
            1056,
            "gnome-terminal-server.Gnome-terminal".to_string(),
            "Terminal".to_string(),
        );

        assert_eq!(window.id, WindowId(83886086));
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
                assert_eq!(window.id, WindowId(83886086));
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
            WindowId(0x05000006),
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
