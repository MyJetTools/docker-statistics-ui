use serde::*;

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    pub src_urls: Vec<String>,
    pub env_name: String,
}

impl SettingsReader {
    pub async fn get_src_urls(&self) -> Vec<String> {
        let read_access = self.settings.read().await;
        return read_access.src_urls.clone();
    }
}
