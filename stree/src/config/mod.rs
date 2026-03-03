use std::{fs::{self, create_dir}, path::Path};

use crate::config::credential::Credential;
use serde::{Deserialize, Serialize};

mod credential;

/*
    [TESTING]: Add configuration
    Create system config
    #improvement #high
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub endpoint: Option<String>,
    pub credential: Option<Credential>,
    pub force_path_style: Option<bool>,
    pub region: Option<String>,
}

impl Config {
    pub fn load(path: String, save: bool) -> Config {
        if fs::exists(&path).unwrap() {
           let content = fs::read_to_string(path).unwrap();
           return toml::from_str::<Config>(&content).unwrap();
        }
        let default = Config {
            endpoint: None,
            credential: None,
            force_path_style: Some(false),
            region: None,
        };
        if save {
            default.save(String::from(&path)).unwrap();
        }        
        default
    }

    pub fn save(&self, filename: String) -> Result<(), String> {
        let path = Path::new(&filename);
        let parent = path.parent().unwrap();
        if !parent.exists() && create_dir(parent).is_err() {
            /*
                [TODO]: Create a error handling system
                Change the error handling system on the app
                #improvement #low
            */
            return Err(String::from("An error occurred"));
        }
        fs::write(path, toml::to_string(self).unwrap()).unwrap();
        Ok(())
    }
}
