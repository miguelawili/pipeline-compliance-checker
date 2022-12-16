use serde::{Deserialize, Serialize};
use std::fs;
use toml;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ApplicationConfiguration {
    pub gitlab: Gitlab,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Gitlab {
    pub base_url: String,
    pub access_token: String,
}

impl ApplicationConfiguration {
    pub fn new(config_path: &str) -> ApplicationConfiguration {
        let file_contents = match fs::read_to_string(config_path) {
            Ok(c) => c,
            Err(e) => panic!("Error {}: Error reading file {}", e, config_path),
        };

        let cfg: ApplicationConfiguration = match toml::from_str(file_contents.as_str()) {
            Ok(d) => d,
            Err(e) => {
                panic!(
                    "Error {:?}: Error parsing file as toml {:?}",
                    e, file_contents
                );
            }
        };

        cfg
    }
}
