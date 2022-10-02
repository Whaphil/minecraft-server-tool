use std::process::{Command};

use std::env;

pub fn start_server(version_name: &str, world_name: &str) {
    let debug = match env::var("DEBUG") {
        Ok(val) => val.to_lowercase() == "true",
        Err(_e) => false,
    };

    let command_output: String = match !debug {
        true => match 
           Command::new("java")
                .args(["-jar", &format!("{}.jar",version_name)[..], "--world", &format!("{}",world_name)[..]])
                .output() {
                    Ok(val) => val.status.to_string(),
                    Err(error) => error.to_string()
                }
        
        false => String::from(format!("DEBUG mode is turned on, server will not be started.\n Values were: Version name: {} World name: {}", version_name,world_name)),
    };

    println!("{}",command_output);
}
