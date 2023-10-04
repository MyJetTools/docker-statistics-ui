use rust_extensions::MyTimerTick;

use crate::APP_CTX;

pub struct UpdateMetricsCacheTimer;

#[async_trait::async_trait]
impl MyTimerTick for UpdateMetricsCacheTimer {
    async fn tick(&self) {
        let settings_reader = APP_CTX.get_settings_reader().await;
        let urls = settings_reader.get_src_urls().await;

        for url in urls {
            let statistics = crate::http_client::get_statistics(url.clone()).await;

            if let Err(err) = &statistics {
                println!("Failed to get statistics from {:?}. Err is: {}", url, err);
                continue;
            }

            let statistics = statistics.unwrap();

            APP_CTX
                .metrics_cache
                .update(&statistics.vm, statistics.containers, url)
                .await
        }
    }
}
