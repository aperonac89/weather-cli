use crate::utils;
use anyhow::Result;

pub struct SolarArgs {}

pub async fn main(cfg: &utils::config::Config) -> Result<()> {
    println!("Test: {:?}", cfg);
    Ok(())
}
