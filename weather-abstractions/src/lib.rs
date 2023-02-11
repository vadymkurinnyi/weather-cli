mod builder;
mod models;
pub mod utils;
use std::error::Error as StdError;

pub use builder::*;
use chrono::NaiveDate;
pub use models::*;

use async_trait::async_trait;
/// This trait defines the interface for a Weather Provider.
#[async_trait]
pub trait WeatherProvider {
    /// This method returns weather information for a given location and date.
    ///
    /// # Arguments
    ///
    /// * `address` - A string containing the location's address
    /// * `date` - An optional NaiveDate representing the date to retrieve the weather information for.
    ///
    /// # Returns
    ///
    /// A Result type that contains either the Weather information or a ProviderError.
    async fn get_weather(
        &self,
        address: &str,
        date: Option<NaiveDate>,
    ) -> Result<Weather, Box<dyn StdError + Send + Sync + 'static>>;
}

use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("Error while build provider {0}")]
    Build(String, anyhow::Error),
    #[error("Not spported provider {0}")]
    NotSupport(String),
}
