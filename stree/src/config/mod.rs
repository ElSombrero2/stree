use std::fs;

use crate::config::credential::Credential;
use serde::{Deserialize, Serialize};

mod credential;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    filename: String,
    endpoint: Option<String>,
    credential: Option<Credential>,
}

impl Config {
    pub fn load(path: String) -> Config {
        let filedir = format!("{}{}", path, "/.config/stree.toml");
        if fs::exists(&filedir).unwrap() {
           let content = fs::read_to_string(filedir).unwrap();
           return toml::from_str::<Config>(&content).unwrap();
        }
        let default = Config {
            filename: String::from(&filedir),
            endpoint: None,
            credential: None,
        };
        default.save();        
        default
    }

    pub fn save(self: &Self) {
        fs::write(&self.filename, toml::to_string(self).unwrap()).unwrap();
    }
}
