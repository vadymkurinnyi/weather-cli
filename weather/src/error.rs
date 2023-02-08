use thiserror::Error;
use weather_provider::ProviderError;

use crate::SettingsError;

#[derive(Error, Debug)]
pub enum WeatherError {
    #[error("Provider error")]
    Provider(#[from] ProviderError),
    #[error("Settings error")]
    Settings(#[from] SettingsError),
}
