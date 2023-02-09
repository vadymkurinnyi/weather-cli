use std::{error::Error, ops::Deref};

use weather_provider::{ProviderError, Units, WeatherKind};
use colored::Colorize;
use crate::{commands::WeatherCommandResult, SettingsError};

pub fn result_to_user_output(result: WeatherCommandResult) {
    match result {
        WeatherCommandResult::Weather(args, weather) => {
            let temp = weather.temp.to_string_value(Units::Metric);
            let date = args.date.unwrap_or_else(|| chrono::offset::Utc::now().date_naive());
            let location = args.address;
            let condition = weather.condition;
            let weather_message = match weather.kind {
                WeatherKind::History => format!(
                    "On {date}, the weather in {location} was {condition} with temperature of {temp}."),
                WeatherKind::Current => 
                    format!("Today in {location}, the current weather conditions are {condition} with a temperature of {temp}."),
                WeatherKind::Forecast => 
                    format!("The forecast for {location} for {date} is {condition} with a predicted high temperature of {temp}."),
            };
            println!("{}", weather_message);  
        }
        WeatherCommandResult::ProviderChanged(provider) => {
            println!("Weather provider changed to: '{}'.", provider)
        }
        WeatherCommandResult::SettingsApplied => println!("The changes was applied.")
        ,
        WeatherCommandResult::Reseted => {
            println!("The settings were reset to default.")
        },
        WeatherCommandResult::Info(info) => {
            match info.provider {
                Some(current) => println!("Current provider: {}", current.green().bold()),
                None => println!("{}", "Provider not set".red().bold()),
            }

            let separator = "-".repeat(40);
        for (p, settings) in info.settings {
            println!("\n{}", separator);
            println!("Settings for provider '{}':", p.bold());

            match settings {
                None => println!("{}", "No settings found".red().bold()),
                Some(settings) => {
                    for (field, value) in settings {
                        println!("{}: {}", field.white().bold(), value.green().bold());
                    }
                }
            }
        }
        },
    }
}

pub fn error_to_user_output(error: Box<dyn Error + 'static>) {
    let message = match error.downcast::<ProviderError>() {
        Ok(error) => match error.deref() {
            ProviderError::NotSupport(..)
            | ProviderError::Temperature(..)
            | ProviderError::UnsupportedDate(..)
            | ProviderError::Parse(..)
            | ProviderError::Api(..)
            | ProviderError::MissingConf(..) => error.to_string(),
            ProviderError::Configuration(..) => "Error while reading provider configuration. Check the provider settings.".to_owned(),
            ProviderError::JSON(..) => format!("Error, the provider may changed protocol. {}", error),
            ProviderError::HttpClient(..) => "Unexpected response from the provider. Might be a connection issue.".to_owned(),
        },
        Err(e) => match e.downcast::<SettingsError>() {
            Ok(settings_err) => settings_err.to_string(),
            Err(e) => format!("{:?}", e),
        },
    };
    println!("{}", message.red());
}
