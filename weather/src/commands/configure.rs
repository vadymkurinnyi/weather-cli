use weather_provider::ProviderManager;

use crate::{args::ConfigureArgs, error::WeatherError, Settings};

use super::WeatherCommandResult;

pub async fn execute(
    args: ConfigureArgs,
    provider_manger: &ProviderManager,
) -> Result<WeatherCommandResult, WeatherError> {
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
