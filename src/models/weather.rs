use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherResponse {
    pub name: String,
    pub cord: Coords,
    pub main: Main,
    pub weather: Vec<Weather>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coords {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    pub main: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Main {
    temp: f64,
    feels_like: f64,
    humidity: i32,
}