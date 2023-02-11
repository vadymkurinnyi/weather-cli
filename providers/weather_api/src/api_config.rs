use crate::{generate_functions, utils::build_endpoint, WeatherApiError};
use reqwest::Url;
use serde::{Deserialize, Serialize};

pub const PROVIDER_NAME: &str = "weather-api";

generate_functions! {
    base_url, "http://api.weatherapi.com",
    current_path, "/v1/current.json",
    history_path, "/v1/history.json",
    forecast_path, "/v1/forecast.json",
    future_path, "/v1/future.json"
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiConfig {
    pub api_key: Option<String>,
    #[serde(default = "base_url")]
    pub base_url: String,
    #[serde(default = "current_path")]
    pub current_path: String,
    #[serde(default = "history_path")]
    pub history_path: String,
    #[serde(default = "forecast_path")]
    pub forecast_path: String,
    #[serde(default = "future_path")]
    pub future_path: String,
}

pub struct Endpoints {
    pub current: Url,
    pub history: Url,
    pub forecast: Url,
    pub future: Url,
}

impl TryFrom<ApiConfig> for Endpoints {
    type Error = WeatherApiError;
    fn try_from(value: ApiConfig) -> Result<Self, Self::Error> {
        let base_url = Url::parse(&value.base_url).map_err(|e| {
            WeatherApiError::Parse(
                format!("{PROVIDER_NAME}/baseUrl"),
                value.base_url.clone(),
                e.to_string(),
            )
        })?;
        Ok(Self {
            current: build_endpoint(&base_url, &value.current_path),
            history: build_endpoint(&base_url, &value.history_path),
            forecast: build_endpoint(&base_url, &value.forecast_path),
            future: build_endpoint(&base_url, &value.future_path),
        })
    }
}
