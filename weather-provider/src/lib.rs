mod builder;
mod error;
mod models;
pub mod utils;
pub use builder::*;
use chrono::NaiveDate;
use config::Config;
pub use error::*;
pub use models::*;
use std::collections::{hash_map::Entry, HashMap, HashSet};

use async_trait::async_trait;
#[async_trait]
pub trait WeatherProvider {
    async fn get_weather(
        &self,
        address: &str,
        date: Option<NaiveDate>,
    ) -> Result<Weather, ProviderError>;
}
type ProviderBuilder = dyn Fn(&Config) -> Result<Box<dyn WeatherProvider + 'static>, ProviderError>;
pub struct WeatherProviderManager<'a> {
    providers: HashMap<String, Box<dyn WeatherProvider>>,
    builders: HashMap<String, Box<ProviderBuilder>>,
    names: HashSet<String>,
    conf: &'a Config,
}

impl<'a> WeatherProviderManager<'a> {
    pub fn get_list_providers(&self) -> Vec<&str> {
        self.names.iter().map(|c| c.as_str()).collect()
    }
    pub fn is_supported(&self, provider_name: &str) -> Result<(), ProviderError> {
        if self.names.contains(provider_name) {
            return Ok(());
        }
        Err(ProviderError::NotSupport(provider_name.to_string()))
    }
    pub fn get_provider(&mut self, name: &str) -> Result<&dyn WeatherProvider, ProviderError> {
        let entry = self.providers.entry(name.to_string());
        let provider_ref = match entry {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(v) => {
                let builder = self
                    .builders
                    .get(name)
                    .ok_or(ProviderError::NotSupport(name.to_string()))?;
                let provider = (builder)(self.conf)?;
                v.insert(provider)
            }
        };
        Ok(&**provider_ref)
    }
}
