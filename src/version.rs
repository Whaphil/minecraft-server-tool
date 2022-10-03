use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Version {
    name: String,
    path_to_forge_file: String,
    current_mods: Vec<Mod>,
    mod_packs: Vec<Modpack>,
}

#[derive(Serialize, Deserialize)]
pub struct Modpack {
    name: String,
    mods: Vec<Mod>,
}

#[derive(Serialize, Deserialize)]
pub struct Mod {
    name: String,
    path_to_mod: String,
}

fn read_json_from_file<T: DeserializeOwned>(path_to_file: String) -> Option<T> {
    let file_content = match fs::read_to_string(path_to_file) {
        Ok(file_content) => file_content,
        Err(e) => {
            println!("{}", e.to_string());
            String::from("{}")
        }
    };
    let json: Option<T> = match serde_json::from_str(&file_content) {
        Ok(value) => Some(value),
        Err(_e) => None,
    };
    json
}

pub fn load_versions(versions_file_path: Option<String>) -> Vec<Version> {
    let actual_version_path = match versions_file_path {
        Some(path) => path,
        None => String::from("./versions.json"),
    };
    let versions: Option<Vec<Version>> = read_json_from_file(actual_version_path);
    match versions {
        Some(versions) => versions,
        None => vec![],
    }
}
