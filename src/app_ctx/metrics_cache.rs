use std::{collections::BTreeMap, sync::Arc};

use tokio::sync::RwLock;

use crate::http_client::ContainerJsonModel;

pub struct MetricsCache {
    data: RwLock<BTreeMap<String, Arc<Vec<ContainerJsonModel>>>>,
}

impl MetricsCache {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(BTreeMap::new()),
        }
    }

    pub async fn update(&self, vm: String, containers: Vec<ContainerJsonModel>) {
        let mut write_access = self.data.write().await;
        write_access.insert(vm, Arc::new(containers));
    }

    pub async fn get_metrics(&self) -> BTreeMap<String, Arc<Vec<ContainerJsonModel>>> {
        let read_access = self.data.read().await;
        read_access.clone()
    }

    pub async fn get_vm_cpu_and_mem(&self) -> BTreeMap<String, (f64, i64)> {
        let mut result = BTreeMap::new();

        let read_access = self.data.read().await;

        for (vm, data) in read_access.iter() {
            let mut cpu = 0.0;
            let mut mem = 0;

            for itm in data.iter() {
                if let Some(usage) = itm.cpu.usage {
                    cpu += usage;
                }

                if let Some(usage) = itm.mem.usage {
                    mem += usage;
                }
            }

            result.insert(vm.clone(), (cpu, mem));
        }

        result
    }

    pub async fn get_metrics_by_vm(&self, vm: &str) -> Arc<Vec<ContainerJsonModel>> {
        let read_access = self.data.read().await;

        match read_access.get(vm) {
            Some(items) => items.clone(),
            None => Arc::new(vec![]),
        }
    }
}
