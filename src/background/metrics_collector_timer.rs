use rust_extensions::MyTimerTick;

use crate::APP_CTX;

pub struct MetricsCollectorTimer;

#[async_trait::async_trait]
impl MyTimerTick for MetricsCollectorTimer {
    async fn tick(&self) {
        let by_vm = APP_CTX
            .metrics_cache
            .get_cpu_mem_by_vm_and_container()
            .await;
    }
}
