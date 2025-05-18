mod cmd;
mod models;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};

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
    Weather(cmd::weather::WeatherArgs),

    /// Gives information about the solar radiation
    Solar,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = utils::config::Config::new();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Weather(args) => cmd::weather::main(&cfg, args).await?,
        Commands::Solar => cmd::solar::main(&cfg).await?,
    }

    Ok(())
}
