use crate::settings::{Settings, SettingsError};

use super::WeatherCommandResult;

/// Resets the settings of the application to its default values.
///
/// # Errors
///
/// Returns an error if there was an issue resetting the settings.
pub async fn execute() -> Result<WeatherCommandResult, SettingsError> {
    Settings::reset().await?;
    Ok(WeatherCommandResult::Reseted)
}
