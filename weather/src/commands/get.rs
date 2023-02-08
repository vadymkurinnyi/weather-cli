use crate::{args::GetWeatherArgs, SettingsError};
use config::Config;
use std::error::Error;
use weather_provider::ProviderManager;

use super::WeatherCommandResult;

pub async fn execute(
    args: GetWeatherArgs,
    provider_manger: &mut ProviderManager,
    cfg: &Config,
) -> Result<WeatherCommandResult, Box<dyn Error>> {
    let provider_name: String = cfg
        .get_string("provider")
        .map_err(|_| SettingsError::ProviderNotSet)?;
    let provider = provider_manger.get_provider(&provider_name)?;

    let weather = provider.get_weather(&args.address, args.date).await?;
    Ok(WeatherCommandResult::Weather(args, weather))
}
