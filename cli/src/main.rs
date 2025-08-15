use std::env::home_dir;

use stree::config;

fn main() {
    if let Some(dir) = home_dir() {
        let config = config::Config::load(String::from(dir.to_str().unwrap()));
        dbg!(config);
    }
   
}
