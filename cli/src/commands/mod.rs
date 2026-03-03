use clap::{Parser, Subcommand};

pub mod list;

 #[derive(Subcommand, Debug)]
pub enum Command {
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
  Rm {
    #[arg(short, long)]
    bucket: String,
    #[arg(short, long)]
    keys: Vec<String>,
  },
  Upload {
    #[arg(short, long)] 
    path: String,
    #[arg(short, long)]
    key: String,
  },
  Download { 
    #[arg(short, long)]
    bucket: String,
    #[arg(short, long)]
    keys: Option<Vec<String>>,
  },
  Info {
    #[arg(short, long)]
    bucket: String,
    #[arg(short, long)]
    key: String,
  }
}

#[derive(Parser, Debug)]
pub struct Root {
  #[arg(long, short)]
  pub config: Option<String>,
  #[command(subcommand)]
  pub commands: Command,
}
