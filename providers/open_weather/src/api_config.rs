use super::OpenWeatherMap;
use config::Config;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use weather_provider::utils::*;
use weather_provider::*;

pub const PROVIDER_NAME: &str = "open-weather";

generate_functions! {
    base_url, "https://api.openweathermap.org",
    history_base_url, "https://history.api.openweathermap.org",
    weather_path, "/data/2.5/weather",
    history_path, "/data/2.5/history/city",
    forecast_path, "/data/2.5/forecast/daily"
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ApiConfig {
    pub api_key: Option<String>,
    #[serde(default = "base_url")]
    pub base_url: String,
    #[serde(default = "history_base_url")]
    pub history_base_url: String,
    #[serde(default = "weather_path")]
    pub weather_path: String,
    #[serde(default = "history_path")]
    pub history_path: String,
    #[serde(default = "forecast_path")]
    pub forecast_path: String,
}
pub struct Endpoints {
    pub weather: Url,
    pub history: Url,
    pub forecast: Url,
}

impl TryFrom<ApiConfig> for Endpoints {
    type Error = ProviderError;
    fn try_from(value: ApiConfig) -> Result<Self, Self::Error> {
        let base_url = Url::parse(&value.base_url).map_err(|e| {
            ProviderError::Parse(
                format!("{PROVIDER_NAME}/baseUrl"),
                value.base_url.clone(),
                e.to_string(),
            )
        })?;
        let history_base_url = Url::parse(&value.history_base_url).map_err(|e| {
            ProviderError::Parse(
                format!("{PROVIDER_NAME}/historyBaseUrl"),
                value.base_url.clone(),
                e.to_string(),
            )
        })?;
        Ok(Self {
            weather: build_endpoint(&base_url, &value.weather_path),
            history: build_endpoint(&history_base_url, &value.history_path),
            forecast: build_endpoint(&base_url, &value.forecast_path),
        })
    }
}

impl OpenWeatherMap {
    pub fn new(cfg: &Config) -> Result<OpenWeatherMap, ProviderError> {
        let api_conf: ApiConfig = cfg.get(PROVIDER_NAME)?;
        let api_key = api_conf.api_key.clone().ok_or(ProviderError::MissingConf(
            "apiKey".to_string(),
            PROVIDER_NAME.to_string(),
        ))?;
        let endpoints = Endpoints::try_from(api_conf)?;
        Ok(OpenWeatherMap {
            api_key,
            endpoints,
            client: Client::new(),
        })
    }
}
