use std::sync::Arc;

use rust_extensions::AppStates;
use tokio::sync::RwLock;

use crate::settings::SettingsReader;

use super::MetricsCache;
pub struct AppCtxInner {
    settings_reader: Arc<SettingsReader>,
}

pub struct AppCtx {
    inner: RwLock<Option<Arc<AppCtxInner>>>,
    pub metrics_cache: MetricsCache,
    pub app_states: Arc<AppStates>,
}

impl AppCtx {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(None),
            metrics_cache: MetricsCache::new(),
            app_states: Arc::new(AppStates::create_initialized()),
        }
    }

    pub async fn inject_settings(&self, settings_reader: Arc<SettingsReader>) {
        let mut write_access = self.inner.write().await;

        write_access.replace(Arc::new(AppCtxInner {
            settings_reader: settings_reader,
        }));
    }

    pub async fn get_settings_reader(&self) -> Arc<SettingsReader> {
        let read_access = self.inner.read().await;
        read_access.as_ref().unwrap().settings_reader.clone()
    }
}
