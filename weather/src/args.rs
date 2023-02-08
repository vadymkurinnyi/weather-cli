use chrono::NaiveDate;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Weather")]
#[command(author = "Vadym K. <vadym.kruinnyi@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Provide weather from different providers", long_about = None)]
pub struct WeatherCliArgs {
    #[clap(subcommand)]
    pub command: CliCommand,
}

#[derive(Debug, Subcommand)]
pub enum CliCommand {
    Configure(ConfigureArgs),
    Get(GetWeatherArgs),
    Info,
}

#[derive(Debug, Args)]
pub struct ConfigureArgs {
    pub provider: String,
    pub key: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Args)]
pub struct GetWeatherArgs {
    #[arg()]
    pub address: String,
    pub date: Option<NaiveDate>,
}
