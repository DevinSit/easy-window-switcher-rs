use core::str;
use std::process::{Command, Output};

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
