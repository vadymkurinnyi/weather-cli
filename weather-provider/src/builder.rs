use std::{collections::HashMap, error::Error};

use crate::{ProviderError, WeatherProvider};
use std::collections::hash_map::Entry;
type ProviderBuilder =
    Box<dyn FnOnce() -> Result<Box<dyn WeatherProvider + 'static>, Box<dyn Error>>>;
#[derive(Default)]
pub struct ProviderManagerBuilder {
    providers: HashMap<String, Box<dyn WeatherProvider>>,
    builders: HashMap<String, ProviderBuilder>,
}

impl ProviderManagerBuilder {
    pub fn add_provider<P>(mut self, name: impl Into<String>, provider: P) -> Self
    where
        P: WeatherProvider + 'static,
    {
        self.providers.insert(name.into(), Box::new(provider));
        self
    }

    pub fn add_provider_builder<B>(mut self, name: impl Into<String>, builder: B) -> Self
    where
        B: FnOnce() -> Result<Box<dyn WeatherProvider + 'static>, Box<dyn Error>> + 'static,
    {
        self.builders.insert(name.into(), Box::new(builder));
        self
    }
    pub fn build(self) -> ProviderManager {
        ProviderManager {
            providers: self.providers,
            builders: self.builders,
        }
    }
}
pub struct ProviderManager {
    providers: HashMap<String, Box<dyn WeatherProvider>>,
    builders: HashMap<String, ProviderBuilder>,
}
impl ProviderManager {
    pub fn get_list_providers(&self) -> Vec<&str> {
        let mut list: Vec<&str> = self.providers.keys().map(|key| key.as_str()).collect();
        list.extend(self.builders.keys().map(|key| key.as_str()));
        list
    }
    pub fn is_supported(&self, provider_name: &str) -> Result<(), ProviderError> {
        if self.providers.contains_key(provider_name) || self.builders.contains_key(provider_name) {
            return Ok(());
        }
        Err(ProviderError::NotSupport(provider_name.to_string()))
    }
    pub fn get_provider(&mut self, name: &str) -> Result<&dyn WeatherProvider, Box<dyn Error>> {
        let entry = self.providers.entry(name.to_string());
        let provider_ref = match entry {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(v) => {
                let builder = self
                    .builders
                    .remove(name)
                    .ok_or(ProviderError::NotSupport(name.to_string()))?;
                let provider = (builder)()?;
                v.insert(provider)
            }
        };
        Ok(&**provider_ref)
    }
}
