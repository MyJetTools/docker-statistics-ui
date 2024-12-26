use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use my_settings_reader::SettingsReader;
use my_ssh::ssh_settings::SshPrivateKeySettingsModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    pub envs: BTreeMap<String, Vec<VmSettingsModel>>,
    pub ssh_private_keys: Option<HashMap<String, SshPrivateKeySettingsModel>>,
    pub request_pass_key: Option<bool>,
}

impl SettingsModel {
    pub fn get_urls(&self) -> Vec<(String, Vec<String>)> {
        let mut result = Vec::new();

        for (env, env_settings) in self.envs.iter() {
            let fl_urls = env_settings.iter().map(|vm| vm.url.to_string()).collect();

            result.push((env.to_string(), fl_urls));
        }

        result
    }
}

pub struct AppSettingsReader {
    settings: SettingsReader<SettingsModel>,
}

impl AppSettingsReader {
    pub fn new() -> Self {
        Self {
            settings: SettingsReader::new("~/.docker-statistics-ui"),
        }
    }

    pub async fn get_settings(&self) -> Arc<SettingsModel> {
        self.settings.get_settings().await
    }

    pub async fn get_urls(&self) -> Vec<(String, Vec<String>)> {
        let settings = self.settings.get_settings().await;
        settings.get_urls()
    }

    pub async fn get_envs(&self) -> Vec<String> {
        let settings = self.settings.get_settings().await;
        settings.envs.keys().cloned().collect()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SshCredentialsSettingsModel {
    pub cert_path: String,
    pub cert_pass_prase: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VmSettingsModel {
    pub url: String,
}

/*
pub struct SettingsReader {
    settings: Mutex<Option<Arc<SettingsModel>>>,
}

impl SettingsReader {
    pub fn new() -> Self {
        Self {
            settings: Mutex::new(None),
        }
    }

    pub async fn get_settings(&self) -> Arc<SettingsModel> {
        let mut settings_access = self.settings.lock().await;

        loop {
            if let Some(settings_access) = settings_access.clone() {
                return settings_access;
            }

            let file_name = rust_extensions::file_utils::format_path("~/.docker-statistics-ui");

            let content = tokio::fs::read_to_string(file_name.as_str()).await;

            if let Err(err) = &content {
                panic!(
                    "Can not read settings file '{}'. Err:{}",
                    file_name.as_str(),
                    err
                );
            }

            let content = content.unwrap();

            let model: SettingsModel = serde_yaml::from_str(content.as_str()).unwrap();

            *settings_access = Some(model.into());
        }
    }
}
 */
