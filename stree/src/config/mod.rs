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
    endpoint: Option<String>,
    credential: Option<Credential>,
    bucket: Option<String>,
}

impl Config {
    pub fn load(path: String) -> Config {
        if fs::exists(&path).unwrap() {
           let content = fs::read_to_string(path).unwrap();
           return toml::from_str::<Config>(&content).unwrap();
        }
        let default = Config {
            endpoint: None,
            credential: None,
            bucket: None,
        };
        default.save(String::from(&path)).unwrap();        
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
