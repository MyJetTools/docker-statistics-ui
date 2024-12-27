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
