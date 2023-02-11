pub mod configure;
pub mod get;
pub mod info;
pub mod reset;
use weather_abstractions::Weather;

use self::info::Info;
use crate::args::GetWeatherArgs;

///Represents the result of
///executing a weather command.
pub enum WeatherCommandResult {
    ///Represents the result of successfully getting the weather for a given address
    Weather(GetWeatherArgs, Weather),
    ///Represents the result of successfully changing the weather provider. Contains the
    ///name of the new provider.
    ProviderChanged(String),
    ///Represents the result of successfully applying the settings.
    SettingsApplied,
    ///Represents the result of successfully resetting the settings.
    Reseted,
    ///Represents the result of successfully getting information about the current
    ///settings and weather providers.
    Info(Info),
}
