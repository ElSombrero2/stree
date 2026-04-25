use clap::Parser;
use crate::commands::{list::ListOption, Root};
use anyhow::Result;

mod commands;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    /*
        [READY]: Create base command handler
        Create base command handler for basics
        command in stree app
        #cli #improvement #high
     */
    let args = Root::parse();
    let config = utils::config::load_config(args.config);
    
    if let commands::Command::Init { filename } = args.commands {
        let path = filename.unwrap_or(String::from("./.stree.toml"));
        commands::init::init(path);
    } else if let Ok(cfg) = config {
        match args.commands {
            commands::Command::Ls { bucket, limit, marker, prefix } => commands::list::list(&cfg, ListOption { bucket, limit, marker, prefix }).await,
            commands::Command::Info { bucket, key, version } => commands::info::get_info(&cfg, bucket, key, version).await,
            commands::Command::Rm { key, bucket, yes } => commands::delete::remove(&cfg, bucket, key, yes).await,
            commands::Command::Download { bucket, keys, path } => commands::download::download(&cfg, bucket, path, keys).await,
            commands::Command::Upload { file, key, bucket, yes } => commands::upload::upload_file(&cfg, bucket, key, file, yes).await,
            commands::Command::Studio { port } => api::serve().await,
            _ => println!("You have already your .stree.toml file"),
        }
    } else { 
        println!("⚠️ Configuration not found, specify your config file or init it in your folder");
    }

  
    Ok(())
}
