use std::error::Error;

use super::WeatherCommandResult;
use crate::{args::ConfigureArgs, Settings};
use weather_provider::ProviderManager;

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
