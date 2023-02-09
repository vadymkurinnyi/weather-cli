use chrono::NaiveDate;
use config::ConfigError;
use reqwest::Error as ReqwestError;
use thiserror::Error;

use crate::TemperatureError;

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("Not spported provider {0}")]
    NotSupport(String),
    #[error("Http client request error")]
    HttpClient(#[from] ReqwestError),
    #[error("Api error: {0} code: {1}")]
    Api(String, u16),
    #[error("{0} returns unexpected JSON, empty '{1}' section")]
    JSON(String, String),
    #[error("Unexpected temperature")]
    Temperature(#[from] TemperatureError),
    #[error("Unsupported date: {0}")]
    UnsupportedDate(NaiveDate),
    #[error("Configuration error")]
    Configuration(#[from] ConfigError),
    #[error("Error while parsing '{1}', {2}. Change the value in the configuration {0}")]
    Parse(String, String, String),
    #[error("Configuration {0} not found for provider {1}")]
    MissingConf(String, String),
}
