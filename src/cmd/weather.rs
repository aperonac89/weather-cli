use anyhow::Result;
use clap::Args;

use crate::models::weather::WeatherResponse;
use crate::utils::config;

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
pub struct WeatherArgs {
    #[arg(
        short,
        long,
        default_value = "Murcia",
        help = "City name to check the weather"
    )]
    city: String,
    #[arg(
        short,
        long,
        default_value = "metric",
        help = "Units to use(metric/imperial/standard)"
    )]
    units: String,
    #[arg(short, long, default_value = "es", help = "Language to use")]
    lang: String,
}

pub async fn main(cfg: &config::Config, args: &WeatherArgs) -> Result<()> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units={}&lang={}",
        args.city, cfg.api_key, args.units, args.lang
    );
    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        println!("{:?}", response.json::<WeatherResponse>().await?);
    }

    Ok(())
}
