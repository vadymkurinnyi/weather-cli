use std::{collections::HashMap, error::Error};

use crate::{ProviderError, WeatherProvider};
use std::collections::hash_map::Entry;
type ProviderBuilder =
    Box<dyn FnOnce() -> Result<Box<dyn WeatherProvider + 'static>, Box<dyn Error>>>;

/// The `ProviderManagerBuilder` struct is used to build the `ProviderManager`.
/// It holds a HashMap of weather provider instances and a HashMap of functions to build weather provider instances.
#[derive(Default)]
pub struct ProviderManagerBuilder {
    providers: HashMap<String, Box<dyn WeatherProvider>>,
    builders: HashMap<String, ProviderBuilder>,
}

impl ProviderManagerBuilder {
    /// Adds a weather provider instance to the `providers` HashMap.
    ///
    /// # Arguments
    /// * `name` - The name of the provider.
    /// * `provider` - An instance of the weather provider to be added.
    ///
    /// # Example
    /// ```
    /// let builder = ProviderManagerBuilder::default();
    /// let provider = MyWeatherProvider::new();
    /// let new_builder = builder.add_provider("MyWeatherProvider", provider);
    /// ```
    #[cfg(not(doctest))]
    pub fn add_provider<P>(mut self, name: impl Into<String>, provider: P) -> Self
    where
        P: WeatherProvider + 'static,
    {
        self.providers.insert(name.into(), Box::new(provider));
        self
    }
    /// Adds a function to build a weather provider instance to the `builders` HashMap.
    ///
    /// # Arguments
    /// * `name` - The name of the provider.
    /// * `builder` - A function that returns a weather provider instance.
    ///
    /// # Example
    /// ```
    /// let builder = ProviderManagerBuilder::default();
    /// let new_builder = builder.add_provider_builder("MyWeatherProvider", || {
    ///     Ok(Box::new(MyWeatherProvider::new()))
    /// });
    /// ```
    #[cfg(not(doctest))]
    pub fn add_provider_builder<B>(mut self, name: impl Into<String>, builder: B) -> Self
    where
        B: FnOnce() -> Result<Box<dyn WeatherProvider + 'static>, Box<dyn Error>> + 'static,
    {
        self.builders.insert(name.into(), Box::new(builder));
        self
    }
    /// Builds a `ProviderManager` instance from the `ProviderManagerBuilder`.
    ///
    /// # Example
    /// ```
    /// let builder = ProviderManagerBuilder::default();
    /// let provider = MyWeatherProvider::new();
    /// let builder = builder.add_provider("MyWeatherProvider", provider);
    /// let manager = builder.build();
    /// ```
    #[cfg(not(doctest))]
    pub fn build(self) -> ProviderManager {
        ProviderManager {
            providers: self.providers,
            builders: self.builders,
        }
    }
}
/// The `ProviderManager` struct manages the weather providers, both pre-existing and newly built.
/// It holds a `HashMap` of both `providers` and `builders`, with the key as the provider's name and the value as
/// the corresponding provider or builder.
pub struct ProviderManager {
    providers: HashMap<String, Box<dyn WeatherProvider>>,
    builders: HashMap<String, ProviderBuilder>,
}
impl ProviderManager {
    /// This method returns a `Vec` of all the names of the providers, both pre-existing and newly built, stored in the `ProviderManager`.
    pub fn get_list_providers(&self) -> Vec<&str> {
        let mut list: Vec<&str> = self.providers.keys().map(|key| key.as_str()).collect();
        list.extend(self.builders.keys().map(|key| key.as_str()));
        list
    }
    /// This method checks if a provider with the given `provider_name` is supported by the `ProviderManager`.
    /// It returns an `Ok` result with an unit if the provider is supported and a `ProviderError::NotSupport` error if not.
    pub fn is_supported(&self, provider_name: &str) -> Result<(), ProviderError> {
        if self.providers.contains_key(provider_name) || self.builders.contains_key(provider_name) {
            return Ok(());
        }
        Err(ProviderError::NotSupport(provider_name.to_string()))
    }
    /// This method returns a reference to the provider with the given `name` if it exists, either as a pre-existing provider or as a newly built one.
    /// If the provider does not exist in either the `providers` or `builders` `HashMap`, it returns a `ProviderError::NotSupport` error.
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
