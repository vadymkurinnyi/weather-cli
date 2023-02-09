use crate::{args::GetWeatherArgs, SettingsError};
use config::Config;
use std::error::Error;
use weather_provider::ProviderManager;

use super::WeatherCommandResult;

/// Retrieve the weather information based on the given command arguments and the current settings
///
/// # Arguments
///
/// * `args` - The arguments given by the user, including the address and date for weather information
/// * `provider_manager` - The manager for all available weather providers, used to determine which provider to use
/// * `cfg` - The configuration that includes the currently set provider
///
/// # Returns
///
/// A `Result` that either contains the retrieved weather information wrapped in `WeatherCommandResult` or an error
/// indicating the reason for failure.
///
/// # Errors
///
/// This function may return the following errors:
///
/// * `SettingsError::ProviderNotSet` if the provider is not set in the current configuration
/// * An error returned by the `get_provider` method of the `ProviderManager`
/// * An error returned by the `get_weather` method of the selected provider
/// * An error wrapping any unexpected failure, including I/O errors
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
