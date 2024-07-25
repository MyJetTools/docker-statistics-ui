use std::sync::Arc;

use flurl::my_ssh::SshSessionsPool;
use rust_extensions::AppStates;
use tokio::sync::Mutex;

use crate::settings::SettingsReader;

use super::DataCacheByEnv;

use crate::background::UpdateMetricsCacheTimer;
use rust_extensions::MyTimer;

pub struct AppCtx {
    pub data_cache_by_env: Mutex<DataCacheByEnv>,
    pub app_states: Arc<AppStates>,
    pub settings_reader: SettingsReader,
    pub ssh_sessions_pool: Arc<SshSessionsPool>,
}

impl AppCtx {
    pub fn new() -> Self {
        let app_states = Arc::new(AppStates::create_initialized());

        let mut timer_5s = MyTimer::new(std::time::Duration::from_secs(3));

        timer_5s.register_timer(
            "MetricsUpdate",
            std::sync::Arc::new(UpdateMetricsCacheTimer),
        );

        timer_5s.start(app_states.clone(), my_logger::LOGGER.clone());

        Self {
            data_cache_by_env: Mutex::new(DataCacheByEnv::new()),
            app_states,
            settings_reader: SettingsReader::new(),
            ssh_sessions_pool: SshSessionsPool::new().into(),
        }
    }
}
