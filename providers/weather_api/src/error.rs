use chrono::NaiveDate;
use thiserror::Error;
use weather_abstractions::TemperatureError;

#[derive(Debug, Error)]
pub enum WeatherApiError {
    #[error("Unexpected temperature")]
    Temperature(#[from] TemperatureError),
    #[error("Http client request error")]
    HttpClient(#[from] reqwest_middleware::Error),
    #[error("Api error: {0} code: {1}")]
    Api(String, u16),
    #[error("{0} returns unexpected JSON, empty '{1}' section")]
    JSON(String, String),
    #[error("Unsupported date: {0}")]
    UnsupportedDate(NaiveDate),
    #[error("Configuration {0} not found for provider {1}")]
    MissingConf(String, String),
    #[error("Error while parsing '{1}', {2}. Change the value in the configuration {0}")]
    Parse(String, String, String),
}
