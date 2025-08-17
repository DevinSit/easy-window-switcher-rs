use core::str;
use std::process::{Command, Output};

pub fn is_tool_installed(tool: &str) -> bool {
    Command::new(tool)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn call_command(args: &[&str]) -> Output {
    Command::new(args[0])
        .args(&args[1..])
        .output()
        .expect("Failed to execute command")
}

pub fn get_command_output(args: &[&str]) -> String {
    let raw_stdout = call_command(args).stdout;

    str::from_utf8(&raw_stdout)
        .expect("Invalid UTF-8 output")
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_tool_installed_existing_tool() {
        // Test with a tool that should exist on most systems
        assert!(is_tool_installed("ls"));
    }

    #[test]
    fn test_is_tool_installed_nonexistent_tool() {
        // Test with a tool that definitely doesn't exist
        assert!(!is_tool_installed("definitely_not_a_real_tool_12345"));
    }

    #[test]
    fn test_call_command_basic() {
        // Test with a simple command that should work on all systems
        let output = call_command(&["echo", "test"]);
        assert!(output.status.success());

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert_eq!(stdout.trim(), "test");
    }

    #[test]
    fn test_get_command_output() {
        // Test with echo command
        let output = get_command_output(&["echo", "hello world"]);
        assert_eq!(output.trim(), "hello world");
    }

    #[test]
    fn test_get_command_output_multiline() {
        // Test with printf for more controlled output
        let output = get_command_output(&["printf", "line1\nline2"]);
        assert_eq!(output, "line1\nline2");
    }

    #[test]
    #[should_panic(expected = "Failed to execute command")]
    fn test_call_command_invalid_command() {
        // This should panic since the command doesn't exist
        call_command(&["definitely_not_a_real_command_12345"]);
    }
}
