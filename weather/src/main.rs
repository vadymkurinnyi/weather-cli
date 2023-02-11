mod args;
mod commands;
mod settings;
mod user_output;

use anyhow::Result;
use args::{CliCommand, WeatherCliArgs};
use clap::Parser;
use config::Config;
use settings::*;
use user_output::print;
use weather_abstractions::ProviderManagerBuilder;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = WeatherCliArgs::parse();
    print(handle(args).await?);
    Ok(())
}
use commands::WeatherCommandResult;
use std::rc::Rc;
async fn handle(args: WeatherCliArgs) -> Result<WeatherCommandResult, AppError> {
    let conf: Config = Settings::conf().await?;
    let conf = Rc::new(conf);
    let conf_ref1 = Rc::clone(&conf);
    let conf_ref2 = Rc::clone(&conf);
    let mut provider_manger = ProviderManagerBuilder::default()
        .add_provider_builder(open_weather::PROVIDER_NAME, move || {
            let open_weather = open_weather::OpenWeatherMap::new(&conf_ref1)?;
            Ok(Box::new(open_weather))
        })
        .add_provider_builder(weather_api::PROVIDER_NAME, move || {
            let weather_api = weather_api::WeatherApiBuilder::build(&conf_ref2)?;
            Ok(Box::new(weather_api))
        })
        .build();
    let res = match args.command {
        CliCommand::Configure(args) => commands::configure::execute(args, &provider_manger).await?,
        CliCommand::Get(args) => commands::get::execute(args, &mut provider_manger, &conf).await?,
        CliCommand::Reset => commands::reset::execute().await?,
        CliCommand::Info => commands::info::execute(&mut provider_manger, &conf).await?,
    };
    Ok(res)
}

use thiserror::Error;
/// This enum `AppError` defines all the errors that can occur when working with the application.
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Settings")]
    Settings(#[from] SettingsError),
    #[error("Provider manager")]
    ProviderManager(#[from] weather_abstractions::Error),
    #[error("Provider")]
    Provider(#[from] anyhow::Error),
}
