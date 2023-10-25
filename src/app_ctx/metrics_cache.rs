use std::collections::BTreeMap;

use tokio::sync::RwLock;

use crate::{
    models::{ContainerJsonModel, ContainerModel, MetricsByVm, VmModel},
    selected_vm::SelectedVm,
    APP_CTX,
};

#[derive(Clone)]
pub struct ContainersWrapper {
    pub api_url: String,
    pub containers: BTreeMap<String, ContainerModel>,
}

impl Into<ContainerModel> for ContainerJsonModel {
    fn into(self) -> ContainerModel {
        ContainerModel {
            id: self.id,
            image: self.image,
            names: self.names,
            labels: self.labels,
            enabled: self.enabled,
            cpu: self.cpu,
            mem: self.mem,
            cpu_usage_history: None,
            mem_usage_history: None,
        }
    }
}

pub struct MetricsCache {
    data: RwLock<BTreeMap<String, ContainersWrapper>>,
}

impl MetricsCache {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(BTreeMap::new()),
        }
    }

    pub async fn update(&self, vm: &str, containers: Vec<ContainerJsonModel>, api_url: String) {
        let mut src = BTreeMap::new();

        for container in containers {
            src.insert(container.id.clone(), container);
        }

        let mut write_access = self.data.write().await;

        if !write_access.contains_key(vm) {
            write_access.insert(
                vm.to_string(),
                ContainersWrapper {
                    api_url,
                    containers: BTreeMap::new(),
                },
            );
        }

        let by_vm = write_access.get_mut(vm).unwrap();

        remove_not_used_keys_keys(&mut by_vm.containers, &src);

        for (id, container) in src {
            if let Some(usage) = container.cpu.usage {
                let mut metrics_history = APP_CTX.metrics_history.lock().await;

                if !metrics_history.contains_key(&id) {
                    metrics_history.insert(id.to_string(), super::MetricsHistoryWrapper::new());
                }

                let wrapper = metrics_history.get_mut(&id).unwrap();

                wrapper.cpu.add(usage);

                if let Some(usage) = container.mem.usage {
                    wrapper.mem.add(usage);
                }
            }

            if !by_vm.containers.contains_key(&id) {
                by_vm.containers.insert(id.clone(), container.into());
            } else {
                let by_id = by_vm.containers.get_mut(&id).unwrap();
                by_id.update(container);
            }
        }
    }

    pub async fn get_containers(&self) -> BTreeMap<String, Vec<ContainerModel>> {
        let read_access = self.data.read().await;

        let mut result = BTreeMap::new();

        for (vm, items) in read_access.iter() {
            let mut vm_result = Vec::with_capacity(items.containers.len());

            for itm in items.containers.values() {
                vm_result.push(itm.clone());
            }

            result.insert(vm.clone(), vm_result);
        }

        result
    }

    pub async fn get_cpu_by_vm_and_container(&self) -> BTreeMap<String, BTreeMap<String, f64>> {
        let mut result = BTreeMap::new();

        for (vm, wrapper) in self.data.read().await.iter() {
            let mut vm_result = BTreeMap::new();

            for itm in wrapper.containers.values() {
                let cpu_usage = if let Some(usage) = itm.cpu.usage {
                    usage
                } else {
                    0.0
                };

                vm_result.insert(itm.id.to_string(), cpu_usage);
            }

            result.insert(vm.clone(), vm_result);
        }

        result
    }

    pub async fn get_mem_by_vm_and_container(&self) -> BTreeMap<String, BTreeMap<String, i64>> {
        let mut result = BTreeMap::new();

        for (vm, wrapper) in self.data.read().await.iter() {
            let mut vm_result = BTreeMap::new();

            for itm in wrapper.containers.values() {
                let mem_usage = if let Some(usage) = itm.mem.usage {
                    usage
                } else {
                    0
                };

                vm_result.insert(itm.id.to_string(), mem_usage);
            }

            result.insert(vm.clone(), vm_result);
        }

        result
    }

    pub async fn get_vm_cpu_and_mem(&self) -> BTreeMap<String, VmModel> {
        let mut result = BTreeMap::new();

        let read_access = self.data.read().await;

        for (vm, wrapper) in read_access.iter() {
            let mut cpu = 0.0;
            let mut mem = 0;
            let mut mem_limit = 0;
            let mut containers_amount = 0;

            for itm in wrapper.containers.values() {
                if let Some(usage) = itm.cpu.usage {
                    cpu += usage;
                }

                if let Some(usage) = itm.mem.usage {
                    mem += usage;
                }

                if let Some(mem_limit_value) = itm.mem.limit {
                    mem_limit += mem_limit_value;
                }

                if itm.enabled {
                    containers_amount += 1;
                }
            }

            result.insert(
                vm.clone(),
                VmModel {
                    api_url: wrapper.api_url.clone(),
                    cpu,
                    mem,
                    containers_amount,
                    mem_limit,
                },
            );
        }

        result
    }

    pub async fn get_metrics_by_vm(&self, selected_vm: &SelectedVm) -> Vec<MetricsByVm> {
        let read_access = self.data.read().await;

        match selected_vm {
            SelectedVm::All => {
                let mut result = Vec::new();

                for (vm, wrapper) in read_access.iter() {
                    for itm in wrapper.containers.values() {
                        result.push(MetricsByVm {
                            vm: Some(vm.to_string()),
                            url: wrapper.api_url.clone(),
                            container: itm.clone(),
                        });
                    }
                }

                result
            }
            SelectedVm::SingleVm(vm) => match read_access.get(vm) {
                Some(wrapper) => {
                    let mut result: Vec<MetricsByVm> = Vec::with_capacity(wrapper.containers.len());

                    for item in wrapper.containers.values() {
                        result.push(MetricsByVm {
                            vm: None,
                            url: wrapper.api_url.clone(),
                            container: item.clone(),
                        });
                    }

                    result
                }
                None => vec![],
            },
        }
    }
}

fn remove_not_used_keys_keys<TValue, TValue2>(
    current: &mut BTreeMap<String, TValue>,
    src: &BTreeMap<String, TValue2>,
) {
    let mut keys_to_removed = Vec::new();

    for key in current.keys() {
        if !src.contains_key(key) {
            keys_to_removed.push(key.to_string());
        }
    }

    for key_to_remove in keys_to_removed {
        current.remove(&key_to_remove);
    }
}
