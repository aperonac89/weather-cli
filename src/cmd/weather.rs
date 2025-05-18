use anyhow::Result;

use crate::models::weather::WeatherResponse;

pub struct WeatherArgs {

}
// TODO: Get API KEY from config
pub async fn main() -> Result<()> {
    let city = "Murcia";
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, API_KEY);
    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        println!("{:?}", response.json::<WeatherResponse>().await?);
    }

    Ok(())
}