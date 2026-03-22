use clap::{Parser, Subcommand};

pub mod list;
pub mod info;
pub mod delete;
pub mod download;

///{n}   _________ __                         
///{n}  /   _____//  |________   ____   ____  
///{n}  \_____  \\   __\_  __ \_/ __ \_/ __ \ 
///{n}  /        \|  |  |  | \/\  ___/\  ___/ 
///{n} /_______  /|__|  |__|    \___  >\___  >
///{n}         \/                   \/     \/ 
///{n} 
///    Stree is a free open source S3 client
///{n} Visit https://github.com/ElSombrero2/stree
#[derive(Subcommand, Debug)]
pub enum Command {
  /// List buckets or files inside your bucket
  Ls {
    #[arg(short, long)]
    bucket: Option<String>,
    #[arg(short, long)]
    limit: Option<i32>,
    #[arg(short, long)]
    marker: Option<String>,
    #[arg(short, long)]
    prefix: Option<String>,
  },
  /// Remove a file from a bucket
  Rm {
    #[arg(short, long)]
    key: Option<String>,
    #[arg(short, long)]
    bucket: String,
  },
  /// Upload a new file inside a bucket
  Upload {
    #[arg(short, long)] 
    path: String,
    #[arg(short, long)]
    key: String,
  },
  /// Download a file
  Download { 
    #[arg(short, long)]
    bucket: String,
    #[arg(short, long)]
    keys: Vec<String>,
    #[arg(short, long)]
    path: String,
  },
  /// Get file information
  Info {
    #[arg(short, long)]
    bucket: String,
    #[arg(short, long)]
    key: String,
    #[arg(short, long)]
    version: Option<String>,
  },
  /// Open the web studio on the specified port (6060 by default)
  Studio {
    #[arg(short, long)]
    port: String,
  }
}

#[derive(Parser, Debug)]
pub struct Root {
  #[arg(long, short)]
  /// Path to your config file (~/.config/stree/config.toml by default)
  pub config: Option<String>,
  #[command(subcommand)]
  pub commands: Command,
}
