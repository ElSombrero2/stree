use std::env::home_dir;
use stree::config::{self, Config};

pub fn load_config(file: Option<String>) -> Option<Config> {
  if let Some(dir) = home_dir() {
    let config_file = file.clone().unwrap_or(dir.to_str().unwrap().to_owned() + "/.config/stree/config.toml");
    let config = config::Config::load(config_file, file.is_none());
    return Some(config);
  }
  None
}
