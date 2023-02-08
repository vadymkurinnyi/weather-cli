use config::Config;
use reqwest::Client;

use super::{
    api_config::{ApiConfig, Endpoints},
    WeatherApi, PROVIDER_NAME,
};
use crate::ProviderError;

pub struct WeatherApiBuilder;
impl WeatherApiBuilder {
    pub fn build(cfg: &Config) -> Result<WeatherApi, ProviderError> {
        let api_conf: ApiConfig = cfg.get(PROVIDER_NAME)?;
        let api_key = api_conf.api_key.clone().ok_or(ProviderError::MissingConf(
            "apiKey".to_string(),
            PROVIDER_NAME.to_string(),
        ))?;
        let endpoints = Endpoints::try_from(api_conf)?;
        Ok(WeatherApi {
            api_key,
            endpoints,
            client: Client::new(),
        })
    }
}
