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
    pub user_groups: Option<HashMap<String, Vec<String>>>,
    pub users: Option<HashMap<String, String>>,
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

    pub fn get_envs(&self, user_id: &str) -> Vec<String> {
        let group_id = match self.users.as_ref() {
            Some(users) => {
                let user_group = users.get(user_id);
                if user_group.is_none() {
                    return Vec::new();
                }

                user_group.unwrap()
            }
            None => return self.envs.keys().cloned().collect(),
        };

        if group_id == "*" {
            return self.envs.keys().cloned().collect();
        }

        let mut allowed_envs = match self.user_groups.as_ref() {
            Some(user_groups) => {
                let envs = user_groups.get(group_id);
                if envs.is_none() {
                    return Vec::new();
                }

                envs.unwrap().clone()
            }
            None => return vec![],
        };

        allowed_envs.retain(|env| self.envs.contains_key(env));

        allowed_envs
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

    /*
    pub async fn get_envs(&self) -> Vec<String> {
        let settings = self.settings.get_settings().await;
        settings.envs.keys().cloned().collect()
    }
     */
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
