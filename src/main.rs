mod cmd;
mod utils;
mod models;

use anyhow::Result;
use clap::{Parser,Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Gives information about the weather
    Weather,

    /// Gives information about the solar radiation
    Solar,
}

#[tokio::main]
async fn main()  -> Result<()>{
    let cli = Cli::parse();

    match &cli.command {
        Commands::Weather => cmd::weather::main().await?,
        Commands::Solar => cmd::solar::main().await?,
    }

    Ok(())
}
