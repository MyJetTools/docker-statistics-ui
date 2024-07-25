use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use flurl::{my_ssh::SshCredentials, FlUrl};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    pub envs: BTreeMap<String, Vec<VmSettingsModel>>,
    pub ssh_credentials: Option<HashMap<String, SshCredentialsSettingsModel>>,
}

impl SettingsModel {
    pub async fn get_fl_urls(&self) -> BTreeMap<String, Vec<(String, FlUrl)>> {
        let mut result = BTreeMap::new();

        for (env_id, vms) in &self.envs {
            let mut fl_urls = Vec::with_capacity(vms.len());

            for vm in vms {
                let fl_url = vm.get_fl_url(self.ssh_credentials.as_ref()).await;

                fl_urls.push(fl_url);
            }

            result.insert(env_id.clone(), fl_urls);
        }

        result
    }

    pub async fn get_fl_url(&self, env: &str, url: &str) -> (String, FlUrl) {
        let vms = self.envs.get(env).unwrap();

        for vm in vms {
            if vm.url.contains(url) {
                return vm.get_fl_url(self.ssh_credentials.as_ref()).await;
            }
        }

        panic!("Can not find vm with env {} and url '{}'", env, url);
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

impl VmSettingsModel {
    pub async fn get_url(
        &self,
        ssh_credentials: Option<&HashMap<String, SshCredentialsSettingsModel>>,
    ) -> (Option<SshCredentials>, String) {
        super::ssh_settings::parse_url(self.url.as_str(), ssh_credentials).await
    }

    pub async fn get_fl_url(
        &self,
        ssh_credentials: Option<&HashMap<String, SshCredentialsSettingsModel>>,
    ) -> (String, FlUrl) {
        let (ssh_credentials, url) = self.get_url(ssh_credentials).await;

        let mut fl_url = FlUrl::new(url.as_str());

        if let Some(ssh_credentials) = ssh_credentials {
            fl_url = fl_url.set_ssh_credentials(ssh_credentials);
        }

        return (url, fl_url);
    }
}

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
