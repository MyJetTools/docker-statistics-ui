use rust_extensions::MyTimerTick;

use crate::server::APP_CTX;

pub struct UpdateMetricsCacheTimer;

#[async_trait::async_trait]
impl MyTimerTick for UpdateMetricsCacheTimer {
    async fn tick(&self) {
        let mut stop_watch = rust_extensions::StopWatch::new();
        stop_watch.start();
        let settings = APP_CTX.settings_reader.get_settings().await;

        let mut spawns = Vec::new();
        for (env, urls) in settings.get_fl_urls().await {
            let task = tokio::spawn(async move {
                for (url, fl_url) in urls {
                    let statistics = crate::server::http_client::get_statistics(fl_url).await;

                    if let Err(err) = &statistics {
                        println!(
                            "Failed to get statistics for env {}. Url: {}. Err is: {:?}",
                            env, url, err
                        );
                        continue;
                    }

                    let statistics = statistics.unwrap();

                    println!("Loaded env:{}. Url: {}", env, url);

                    APP_CTX.data_cache_by_env.lock().await.update(
                        &env,
                        &statistics.vm,
                        statistics.containers,
                        url,
                    );
                }
            });

            spawns.push(task);
        }

        for spawn in spawns {
            let _ = spawn.await;
        }

        stop_watch.pause();

        println!("UpdateMetricsCacheTimer took: {:?}", stop_watch.duration());
    }
}
