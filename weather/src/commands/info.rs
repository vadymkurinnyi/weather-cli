use std::{collections::HashMap, error::Error};

use config::Config;
use weather_provider::ProviderManager;

use super::WeatherCommandResult;

/// The execute function is used to retrieve the current configuration information for all available weather providers.
///
/// # Arguments
///
/// * `provider_manger` - An instance of `ProviderManager` that provides the list of providers and their configurations
/// * `cfg` - An instance of `Config` that holds the configuration for the weather providers
///
/// # Returns
///
/// A `Result` that wraps the `WeatherCommandResult` for the Info command.
///
/// # Errors
///
/// An error will be returned if there is a failure in accessing the configurations for the providers
pub async fn execute(
    provider_manger: &mut ProviderManager,
    cfg: &Config,
) -> Result<WeatherCommandResult, Box<dyn Error>> {
    let providers = provider_manger.get_list_providers();

    let mut settings = Vec::new();

    for p in providers.into_iter() {
        let provider_conf = cfg.get::<HashMap<String, String>>(p);
        let configuration = match provider_conf {
            Ok(mut configuration) => {
                configuration
                    .entry("apiKey".to_string())
                    .and_modify(hide_sensetive);
                Some(configuration)
            }
            Err(_) => None,
        };

        settings.push((p.to_string(), configuration));
    }
    let provider = cfg.get_string("provider").ok();
    let info = Info { provider, settings };
    Ok(WeatherCommandResult::Info(info))
}

/// Holds the configuration information for the weather providers
///
/// # Fields
///
/// * `provider` - A `String` that holds the name of the current provider being used.
/// * `settings` - A `Vec` of `(String, Option<HashMap<String, String>>)` tuples, where each tuple represents a weather provider and its configuration, if any.
pub struct Info {
    pub provider: Option<String>,
    pub settings: Vec<(String, Option<HashMap<String, String>>)>,
}

fn hide_sensetive(s: &mut String) {
    let len = s.len();
    let hide = match len {
        4.. => (len - 4).max(len / 2),
        _ => len / 2,
    };
    s.replace_range(0..hide, &"*".repeat(hide));
}
