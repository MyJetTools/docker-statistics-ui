use std::collections::BTreeMap;

use crate::models::ContainerJsonModel;

use super::DataCache;

pub struct DataCacheByEnv {
    pub envs: BTreeMap<String, DataCache>,
}

impl DataCacheByEnv {
    pub fn new() -> Self {
        Self {
            envs: BTreeMap::new(),
        }
    }

    pub fn update(
        &mut self,
        env: &str,
        vm: &str,
        containers: Vec<ContainerJsonModel>,
        api_url: String,
    ) {
        if !self.envs.contains_key(env) {
            self.envs.insert(env.to_string(), DataCache::new());
        }

        self.envs
            .get_mut(env)
            .unwrap()
            .update(vm, containers, api_url);
    }
}
