use config::Config;
use serde::Serialize;
use serde_json::{json, Value};
use std::path::PathBuf;
use tokio::{
    fs::{create_dir_all, read_to_string, File},
    io::AsyncWriteExt,
};
pub const APP_NAME: &str = "weather";

pub struct Settings {}
impl Settings {
    pub async fn conf() -> Result<Config, SettingsError> {
        let congif_path = get_conf_path().await;
        if !congif_path.exists() {
            let mut file = File::create(congif_path.as_path()).await?;
            file.write_all("{}".as_bytes()).await?;
        }
        let conf = Config::builder()
            .add_source(config::File::from(congif_path))
            .build()
            .expect("Configuration has to be constructed.");
        Ok(conf)
    }
    pub async fn set<T>(path: &str, val: &T) -> Result<(), SettingsError>
    where
        T: Sized + Serialize,
    {
        let congif_path = get_conf_path().await;
        let file_contents = read_to_string(congif_path.as_path()).await?;
        let mut config: Value =
            serde_json::from_str(&file_contents).map_err(|_| SettingsError::Damaged)?;
        let str_val = serde_json::to_string(val).map_err(|_| SettingsError::Input)?;
        let val: Value = serde_json::from_str(&str_val).map_err(|_| SettingsError::Input)?;
        dbg!(&config);

        let mut paths: Vec<&str> = path.split('/').collect();
        let last = paths.pop().ok_or(SettingsError::Path(path.to_string()))?;

        let mut current = config.as_object_mut().ok_or(SettingsError::RootConfig)?;

        for sub_path in paths.into_iter() {
            current = current
                .entry(sub_path)
                .or_insert(json!({}))
                .as_object_mut()
                .ok_or(SettingsError::Structure(sub_path.to_owned()))?;
        }
        current.insert(last.to_owned(), val);

        let str_conf = serde_json::to_string(&config).map_err(|_| SettingsError::Damaged)?;
        File::create(congif_path.as_path())
            .await?
            .write_all(str_conf.as_bytes())
            .await?;
        Ok(())
    }
}

use directories::ProjectDirs;
async fn get_conf_path() -> PathBuf {
    let project = ProjectDirs::from("com", "", APP_NAME)
        .expect("Unable to find the path of the home directory.");
    let congif_dir = project.config_dir().to_owned();
    if !congif_dir.exists() {
        create_dir_all(congif_dir.as_path())
            .await
            .expect("Configuration directoy should be created.");
    }
    congif_dir.join("settings.json")
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("Not valid settings name {0}")]
    Path(String),
    #[error("Unable to set value, given path '{0}' isn't an JSON object")]
    Structure(String),
    #[error("Root config isn't an JSON object")]
    RootConfig,
    #[error("The value is invalid for setting in the settings.")]
    Input,
    #[error("File system error. Unable to read/write a settings file")]
    IO(#[from] std::io::Error),
    #[error("Unexpected content in a setting file. Reset the settings")]
    Damaged,
    #[error("Provider not set. Please configure the provider")]
    ProviderNotSet,
}
