pub mod configure;
pub mod get;
pub mod info;
use self::info::Info;
use crate::args::GetWeatherArgs;
use weather_provider::Weather;

pub enum WeatherCommandResult {
    Weather(GetWeatherArgs, Weather),
    ProviderChanged(String),
    SettingsApplied,
    Info(Info),
}
