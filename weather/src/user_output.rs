use std::{error::Error, ops::Deref};

use weather_provider::{ProviderError, Units, WeatherKind};
use colored::Colorize;
use crate::{commands::WeatherCommandResult, SettingsError};

pub fn result_to_user_output(result: WeatherCommandResult) {
    match result {
        WeatherCommandResult::Weather(args, weather) => {
            let temp = weather.temp.to_string_value(Units::Metric);
            let date = args.date.unwrap_or(chrono::offset::Utc::now().date_naive());
            let location = args.address;
            let condition = weather.condition;
            let weather = match weather.kind {
                WeatherKind::History => format!(
                    "On {date}, the weather in {location} was {condition} with temperature of {temp}."),
                WeatherKind::Current => 
                    format!("Today in {location}, the current weather conditions are {condition} with a temperature of {temp}."),
                WeatherKind::Forecast => 
                    format!("The forecast for {location} for {date} is {condition} with a predicted high temperature of {temp}."),
            };
            println!("{}", weather);  
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
                Some(current) => println!("Current provider: {}", current.green()),
                None => println!("{}", "Provider not set".red()),
            }

            let line = std::iter::repeat("-").take(20).collect::<String>();
            for (p, settings) in info.settings {
                match settings {
                    None => {
                        println!("Settings for provider '{}' is empty.", p)
                    },
                    Some(settings) => {
                        println!("{} settings: ", p);
                        for (field, value) in settings {
                            println!("{}: {}", field.white(), value.green() )
                        }
                    },
                    
                }
                println!("{}", line);
            }
        },
    }
}

pub fn error_to_user_output(error: Box<dyn Error + 'static>) {

    match error.downcast::<ProviderError>() {
        Ok(error) =>{
            let error = error.deref();
            match error {
                ProviderError::NotSupport(_, ..)
                | ProviderError::Temperature(_, ..)
                | ProviderError::UnsupportedDate(_, ..)
                | ProviderError::Parse(_, ..)
                | ProviderError::Api(_, ..)
                | ProviderError::MissingConf(_, ..) => output(error.to_string()),
                ProviderError::Configuration(_, ..) => {
                    output("Error while reading provider configuration. Check the provider settings.")
                }
                ProviderError::JSON(_, ..) => output(format!(
                    "Error, the provider may changed protocol. {error}"
                )),
                ProviderError::HttpClient(_, ..) => {
                    output("Unexpected response from the provider. Might be a connection issue.")
                }
            }
        },
        Err(e) => {
            match e.downcast::<SettingsError>() {
                Ok(settings_err) => output(settings_err.to_string()),
                Err(e) => println!("{:?},", e)}
            }
            
    }

    
}

fn output(message: impl Into<String>) {
    println!("{}", message.into().red());
}
