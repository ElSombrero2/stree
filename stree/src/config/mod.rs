use std::fs;

use crate::config::credential::Credential;
use anyhow::{anyhow, Result};
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
    pub fn load(path: String) -> Result<Config> {
        if fs::exists(&path).unwrap() {
           let content = fs::read_to_string(path).unwrap();
           return Ok(toml::from_str::<Config>(&content).unwrap());
        }
        Err(anyhow!("Missing config"))
    }
}
