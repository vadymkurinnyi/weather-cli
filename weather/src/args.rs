use chrono::NaiveDate;
use clap::{Args, Parser, Subcommand};

/// Weather command line interface (CLI) arguments.
#[derive(Parser, Debug)]
#[command(name = "Weather")]
#[command(author = "Vadym K. <vadym.kruinnyi@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Provide weather from different providers", long_about = None)]
pub struct WeatherCliArgs {
    #[clap(subcommand)]
    pub command: CliCommand,
}

/// An enumeration of the different sub-commands available for the weather CLI.
#[derive(Debug, Subcommand)]
pub enum CliCommand {
    Configure(ConfigureArgs),
    Get(GetWeatherArgs),
    ///Retrieve the current configuration information for all available weather providers
    Info,
    ///Resets the settings of the application to its default values.
    Reset,
}

/// Configuration arguments for the weather CLI
#[derive(Debug, Args)]
pub struct ConfigureArgs {
    /// The name of the desired weather provider
    pub provider: String,
    /// The key for the desired setting, for example: apiKey
    pub key: Option<String>,
    /// The value for the specified setting key
    pub value: Option<String>,
}

/// Retrieve the weather information
#[derive(Debug, Args)]
pub struct GetWeatherArgs {
    /// A string containing the location's address    
    pub address: String,
    /// An optional NaiveDate representing the date to retrieve the weather information for
    pub date: Option<NaiveDate>,
}
