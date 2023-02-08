use crate::{args::GetWeatherArgs, error::WeatherError, SettingsError};
use config::Config;
use weather_provider::{ProviderError, ProviderManager};

use super::WeatherCommandResult;

pub async fn execute(
    args: GetWeatherArgs,
    provider_manger: &mut ProviderManager,
    cfg: &Config,
) -> Result<WeatherCommandResult, WeatherError> {
    let provider_name: String = cfg
        .get_string("provider")
        .map_err(|_| SettingsError::ProviderNotSet)?;
    let provider = provider_manger
        .get_provider(&provider_name)
        .map_err(|_| ProviderError::NotSupport(String::new()))?;

    let weather = provider.get_weather(&args.address, args.date).await?;
    // println!("Weather {:?}", weather.temp);
    Ok(WeatherCommandResult::Weather(args, weather))
}
