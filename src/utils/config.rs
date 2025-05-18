use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            api_key: env::var("API_KEY").expect("API_KEY must be set"),
        }
    }
}
