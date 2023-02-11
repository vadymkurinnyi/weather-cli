use std::error::Error;

use config::Config;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use reqwest_tracing::TracingMiddleware;

use crate::WeatherApiError;

use super::{
    api_config::{ApiConfig, Endpoints},
    WeatherApi, PROVIDER_NAME,
};

pub struct WeatherApiBuilder;
impl WeatherApiBuilder {
    pub fn build(cfg: &Config) -> Result<WeatherApi, Box<dyn Error + Send + Sync>> {
        let api_conf: ApiConfig = cfg.get(PROVIDER_NAME)?;
        let api_key = api_conf
            .api_key
            .clone()
            .ok_or(WeatherApiError::MissingConf(
                "apiKey".to_string(),
                PROVIDER_NAME.to_string(),
            ))?;
        let endpoints = Endpoints::try_from(api_conf)?;

        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(TracingMiddleware::default())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();
        Ok(WeatherApi {
            api_key,
            endpoints,
            client,
        })
    }
}
