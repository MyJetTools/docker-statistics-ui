use std::{collections::HashMap, sync::Arc};

use rust_extensions::AppStates;
use tokio::sync::Mutex;

use crate::settings::SettingsModel;

use super::{MetricsCache, MetricsHistory};

use crate::background::UpdateMetricsCacheTimer;
use rust_extensions::MyTimer;
pub struct MetricsHistoryWrapper {
    pub cpu: MetricsHistory<f64>,
    pub mem: MetricsHistory<i64>,
}
impl MetricsHistoryWrapper {
    pub fn new() -> Self {
        Self {
            cpu: MetricsHistory::new(),
            mem: MetricsHistory::new(),
        }
    }
}

pub struct AppCtx {
    pub metrics_cache: MetricsCache,
    pub app_states: Arc<AppStates>,
    pub settings: SettingsModel,

    pub metrics_history: Mutex<HashMap<String, MetricsHistoryWrapper>>,
}

impl AppCtx {
    pub fn new() -> Self {
        let settings = SettingsModel;

        let app_states = Arc::new(AppStates::create_initialized());

        let mut timer_5s = MyTimer::new(std::time::Duration::from_secs(5));

        timer_5s.register_timer(
            "MetricsUpdate",
            std::sync::Arc::new(UpdateMetricsCacheTimer),
        );

        timer_5s.start(app_states.clone(), my_logger::LOGGER.clone());

        Self {
            metrics_cache: MetricsCache::new(),
            app_states,
            settings,
            metrics_history: Mutex::new(HashMap::new()),
        }
    }
}
