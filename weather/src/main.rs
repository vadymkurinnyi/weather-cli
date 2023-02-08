mod args;
mod commands;
mod error;
mod settings;
mod user_output;

use args::{CliCommand, WeatherCliArgs};
use clap::Parser;
use config::Config;
use error::WeatherError;
pub use settings::*;
use std::rc::Rc;
use user_output::{error_to_user_output, result_to_user_output};
use weather_provider::ProviderManagerBuilder;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = WeatherCliArgs::parse();
    match handle(args).await {
        Ok(result) => result_to_user_output(result),
        Err(e) => error_to_user_output(e),
    };
}
use commands::WeatherCommandResult;
async fn handle(args: WeatherCliArgs) -> Result<WeatherCommandResult, WeatherError> {
    let conf: Config = Settings::conf().await?;
    let conf = Rc::new(conf);
    let conf_clone = Rc::clone(&conf);
    let open_weather = open_weather::OpenWeatherMap::new(&conf)?;
    let mut provider_manger = ProviderManagerBuilder::new()
        .add_provider(open_weather::PROVIDER_NAME, open_weather)
        .add_provider_builder(weather_api::PROVIDER_NAME, move || {
            let weather_api = weather_api::WeatherApiBuilder::build(&conf_clone)?;
            Ok(Box::new(weather_api))
        })
        .build();
    let res = match args.command {
        CliCommand::Configure(args) => commands::configure::execute(args, &provider_manger).await?,
        CliCommand::Get(args) => commands::get::execute(args, &mut provider_manger, &conf).await?,
        CliCommand::Info => commands::info::execute(&mut provider_manger, &conf).await?,
    };
    Ok(res)
}

// fn test() {
//     let result: Result<T, Box<dyn std::error::Error>> = some_function();
//     match result {
//         Ok(val) => { /* handle success */ }
//         Err(e) => match e.downcast::<WeatherError>() {
//             Ok(my_error) => { /* handle MyErrorType */ }
//             Err(_) => { /* handle other errors */ }
//         },
//     }
// }
