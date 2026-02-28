use std::env::home_dir;

use stree::config;

fn main() {
    let default_config_file = "/.config/stree/config.toml";

    if let Some(dir) = home_dir() {
        let config_dir =  dir.to_str().unwrap().to_owned() + default_config_file;
        println!("{config_dir}");
        let config = config::Config::load(config_dir);
        dbg!(config);
    }
   
}
