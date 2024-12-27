use rust_extensions::MyTimerTick;

use crate::server::APP_CTX;

pub struct UpdateMetricsCacheTimer;

#[async_trait::async_trait]
impl MyTimerTick for UpdateMetricsCacheTimer {
    async fn tick(&self) {
        let mut stop_watch = rust_extensions::StopWatch::new();
        stop_watch.start();
        let urls = crate::server::APP_CTX.settings_reader.get_urls().await;

        let mut spawns = Vec::new();
        for (env, urls) in urls {
            let task = tokio::spawn(async move {
                for api_url in urls {
                    let fl_url = crate::server::APP_CTX.create_fl_url(&api_url);

                    let statistics = crate::server::http_client::get_statistics(fl_url).await;

                    if let Err(err) = &statistics {
                        println!(
                            "Failed to get statistics for env {}. Url: {}. Err is: {:?}",
                            env, api_url, err
                        );
                        continue;
                    }

                    let statistics = statistics.unwrap();

                    println!("Loaded env:{}. Url: {}", env, api_url);

                    {
                        let mut data_cache = APP_CTX.data_cache_by_env.lock().await;

                        data_cache.update(&env, &statistics.vm, statistics.containers, api_url);
                    }
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
