use clap::Parser;
use crate::commands::{Root, list::ListOption};
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

    if let Some(cfg) = config {
        match args.commands {
            commands::Command::Ls { bucket, limit, marker, prefix } => {
                commands::list::list(&cfg, ListOption { bucket, limit, marker, prefix }).await?;
            }
            commands::Command::Rm { bucket, keys } => {
                dbg!(keys);
                dbg!(bucket);
            },
            commands::Command::Download { bucket: _, keys: _ } => todo!(""),
            _ => todo!("")
        }
    }

  
    Ok(())
}
