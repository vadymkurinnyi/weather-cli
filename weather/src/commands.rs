pub mod configure;
pub mod get;
pub mod info;
pub mod reset;
use weather_provider::Weather;

use self::info::Info;
use crate::args::GetWeatherArgs;

pub enum WeatherCommandResult {
    Weather(GetWeatherArgs, Weather),
    ProviderChanged(String),
    SettingsApplied,
    Reseted,
    Info(Info),
}
