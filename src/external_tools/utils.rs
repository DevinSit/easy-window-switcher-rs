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
