use std::error::Error;

use super::WeatherCommandResult;
use crate::{args::ConfigureArgs, Settings};
use weather_provider::ProviderManager;

/// Execute the configure command.
///
/// This function configures the desired provider and sets its API key or other provider settings.
///
/// # Arguments
///
/// * `args` - The arguments for the configure command.
/// * `provider_manager` - A reference to the ProviderManager.
///
/// # Returns
///
/// If the configuration was successful, this function returns a WeatherCommandResult enum value, indicating
/// the action taken. If there was an error, the function returns a `Result` type with an error value of type
/// `Box<dyn Error>`.
///
/// # Errors
///
/// This function can return the following errors:
///
/// * If the provider specified in the arguments is not supported.
/// * If an error occurred while setting the API key or provider in the settings.
pub async fn execute(
    args: ConfigureArgs,
    provider_manger: &ProviderManager,
) -> Result<WeatherCommandResult, Box<dyn Error>> {
    provider_manger.is_supported(&args.provider)?;
    match args {
        ConfigureArgs {
            key: Some(key),
            value: Some(value),
            provider,
        } => {
            let path = format!("{provider}/{key}");
            Settings::set(&path, &value).await?;
            Ok(WeatherCommandResult::SettingsApplied)
        }
        ConfigureArgs { provider, .. } => {
            Settings::set("provider", &provider).await?;
            Ok(WeatherCommandResult::ProviderChanged(provider))
        }
    }
}
