use weather_abstractions::{ Units, WeatherKind };
use colored::Colorize;
use crate::commands::WeatherCommandResult;

pub fn print(result: WeatherCommandResult) {
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