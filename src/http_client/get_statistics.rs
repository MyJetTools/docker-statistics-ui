use std::collections::BTreeMap;

use flurl::IntoFlUrl;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StatisticsContract {
    pub vm: String,
    pub containers: Vec<ContainerJsonModel>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ContainerJsonModel {
    pub id: String,
    pub image: String,
    pub names: Vec<String>,
    pub labels: Option<BTreeMap<String, String>>,
    pub enabled: bool,
    pub cpu: CpuUsageJsonMode,
    pub mem: MemUsageJsonMode,
}

impl ContainerJsonModel {
    pub fn filter_me(&self, value: &str) -> bool {
        if value == "" {
            return true;
        }

        if self.id.contains(value) {
            return true;
        }

        let value = value.to_lowercase();

        if self.image.to_lowercase().contains(&value) {
            return true;
        }

        for name in &self.names {
            if name.to_lowercase().contains(&value) {
                return true;
            }
        }

        if let Some(labels) = &self.labels {
            for (key, v) in labels {
                if key.to_lowercase().contains(&value) {
                    return true;
                }

                if v.to_lowercase().contains(&value) {
                    return true;
                }
            }
        }

        return false;
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CpuUsageJsonMode {
    pub usage: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MemUsageJsonMode {
    pub usage: Option<i64>,
    pub available: Option<i64>,
    pub limit: Option<i64>,
}

pub async fn get_statistics(url: String) -> Result<StatisticsContract, String> {
    let url_response = url
        .append_path_segment("api")
        .append_path_segment("containers")
        .get()
        .await;

    if let Err(err) = &url_response {
        return Err(format!("Error: {:?}", err));
    };

    let result = url_response.unwrap().get_json().await;

    if let Err(err) = &result {
        return Err(format!("Error: {:?}", err));
    };

    Ok(result.unwrap())
}
