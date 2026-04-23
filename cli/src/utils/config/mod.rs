use stree::config::Config;
use anyhow::Result;

pub fn load_config(file: Option<String>) -> Result<Config> {
    let default_path = String::from("./.stree.toml");
    return Config::load(file.unwrap_or(default_path)); 
}
