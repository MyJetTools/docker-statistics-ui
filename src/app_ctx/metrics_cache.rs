use std::{collections::BTreeMap, sync::Arc};

use tokio::sync::RwLock;

use crate::{http_client::ContainerJsonModel, states::SelectedVm};

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

    pub async fn get_cpu_mem_by_vm_and_container(
        &self,
    ) -> BTreeMap<String, BTreeMap<String, (f64, i64)>> {
        let mut result = BTreeMap::new();

        for (vm, items) in self.data.read().await.iter() {
            let mut vm_result = BTreeMap::new();

            for itm in items.iter() {
                let cpu_usage = if let Some(usage) = itm.cpu.usage {
                    usage
                } else {
                    0.0
                };

                let mem_usage = if let Some(usage) = itm.mem.usage {
                    usage
                } else {
                    0
                };

                vm_result.insert(itm.id.to_string(), (cpu_usage, mem_usage));
            }

            result.insert(vm.clone(), vm_result);
        }

        result
    }

    pub async fn get_vm_cpu_and_mem(&self) -> BTreeMap<String, (f64, i64, usize)> {
        let mut result = BTreeMap::new();

        let read_access = self.data.read().await;

        for (vm, data) in read_access.iter() {
            let mut cpu = 0.0;
            let mut mem = 0;
            let mut amount = 0;

            for itm in data.iter() {
                if let Some(usage) = itm.cpu.usage {
                    cpu += usage;
                }

                if let Some(usage) = itm.mem.usage {
                    mem += usage;
                }

                if itm.enabled {
                    amount += 1;
                }
            }

            result.insert(vm.clone(), (cpu, mem, amount));
        }

        result
    }

    pub async fn get_metrics_by_vm(
        &self,
        selected_vm: &SelectedVm,
    ) -> Vec<(Option<String>, ContainerJsonModel)> {
        let read_access = self.data.read().await;

        match selected_vm {
            SelectedVm::All => {
                let mut result = Vec::new();

                for (vm, items) in read_access.iter() {
                    for itm in items.iter() {
                        result.push((Some(vm.to_string()), itm.clone()));
                    }
                }

                result
            }
            SelectedVm::SingleVm(vm) => match read_access.get(vm) {
                Some(items) => {
                    let mut result: Vec<(Option<String>, ContainerJsonModel)> =
                        Vec::with_capacity(items.len());

                    for item in items.as_slice() {
                        result.push((None, item.clone()));
                    }

                    result
                }
                None => vec![],
            },
        }
    }
}
