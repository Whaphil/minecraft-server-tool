use std::process::{Command, Output};

use std::env;

fn parse_output(output: Output) -> String {
    match output.status.code() {
        None => String::from("No exit code"),
        Some(code) => match code {
            0 => String::from_utf8(output.stdout).unwrap_or(String::from(
                "Someting went wrong trying to parse the output",
            )),
            _ => String::from_utf8(output.stderr).unwrap_or(String::from(
                "Somthing went wrong trying to parse the error message",
            )),
        },
    }
}

fn run_jar_command(version_name: &str, world_name: &str) -> String {
    let output = Command::new("java")
        .args([
            "-jar",
            &format!("{}.jar", version_name)[..],
            "--world",
            &format!("{}", world_name)[..],
        ])
        .output();
    match output {
        Ok(output) => parse_output(output),
        Err(_e) => String::from("Someting went wrong trying to run the program"),
    }
}

fn get_debug_message(version_name: &str, world_name: &str) -> String {
    format!("DEBUG mode is turned on, server will not be started.\n Values were: Version name: {} World name: {}", version_name,world_name)
}

pub fn start_server(version_name: &str, world_name: &str) {
    let debug = match env::var("DEBUG") {
        Ok(val) => val.to_lowercase() == "true",
        Err(_e) => false,
    };

    let command_output: String = match !debug {
        true => run_jar_command(version_name, world_name),
        false => get_debug_message(version_name, world_name),
    };

    println!("{}", command_output);
}
