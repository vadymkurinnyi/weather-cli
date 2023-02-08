use std::error::Error;

use crate::settings::Settings;

use super::WeatherCommandResult;

pub async fn execute() -> Result<WeatherCommandResult, Box<dyn Error>> {
    Settings::reset().await?;
    Ok(WeatherCommandResult::Reseted)
}
