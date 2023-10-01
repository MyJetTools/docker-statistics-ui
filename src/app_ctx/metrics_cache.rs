use std::collections::BTreeMap;

use tokio::sync::RwLock;

use crate::{
    http_client::{ContainerJsonModel, CpuUsageJsonMode, MemUsageJsonMode},
    states::SelectedVm,
};

use super::MetricsHistory;

#[derive(Clone)]
pub struct ContainerModel {
    pub id: String,
    pub image: String,
    pub names: Vec<String>,
    pub labels: Option<BTreeMap<String, String>>,
    pub enabled: bool,
    pub cpu: CpuUsageJsonMode,
    pub mem: MemUsageJsonMode,
    pub cpu_usage_history: MetricsHistory<f64>,
    pub mem_usage_history: MetricsHistory<i64>,
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
            cpu_usage_history: MetricsHistory::new(),
            mem_usage_history: MetricsHistory::new(),
        }
    }
}

impl ContainerModel {
    pub fn filter_me(&self, value: &str) -> bool {
        if value == "" {
            return true;
        }

        if self.id.contains(value) {
            return true;
        }

        let value = value.to_lowercase();

        if self.image.to_lowercase().contains(&value) {
            return true;
        }

        for name in &self.names {
            if name.to_lowercase().contains(&value) {
                return true;
            }
        }

        if let Some(labels) = &self.labels {
            for (key, v) in labels {
                if key.to_lowercase().contains(&value) {
                    return true;
                }

                if v.to_lowercase().contains(&value) {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn update(&mut self, src: ContainerJsonModel) {
        if let Some(usage) = src.cpu.usage {
            self.cpu_usage_history.add(usage);
        }

        if let Some(usage) = src.mem.usage {
            self.mem_usage_history.add(usage);
        }
        self.cpu = src.cpu;
        self.mem = src.mem;
        self.labels = src.labels;
        self.enabled = src.enabled;
        self.image = src.image;
    }
}

pub struct MetricsCache {
    data: RwLock<BTreeMap<String, BTreeMap<String, ContainerModel>>>,
}

impl MetricsCache {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(BTreeMap::new()),
        }
    }

    pub async fn update(&self, vm: &str, containers: Vec<ContainerJsonModel>) {
        let mut src = BTreeMap::new();

        for container in containers {
            src.insert(container.id.clone(), container);
        }

        let mut write_access = self.data.write().await;

        if !write_access.contains_key(vm) {
            write_access.insert(vm.to_string(), BTreeMap::new());
        }

        let by_vm = write_access.get_mut(vm).unwrap();

        remove_not_used_keys_keys(by_vm, &src);

        for (id, container) in src {
            if !by_vm.contains_key(&id) {
                by_vm.insert(id.clone(), container.into());
            } else {
                let by_id = by_vm.get_mut(&id).unwrap();
                by_id.update(container);
            }
        }
    }

    pub async fn get_containers(&self) -> BTreeMap<String, Vec<ContainerModel>> {
        let read_access = self.data.read().await;

        let mut result = BTreeMap::new();

        for (vm, items) in read_access.iter() {
            let mut vm_result = Vec::with_capacity(items.len());

            for itm in items.values() {
                vm_result.push(itm.clone());
            }

            result.insert(vm.clone(), vm_result);
        }

        result
    }

    pub async fn get_cpu_by_vm_and_container(&self) -> BTreeMap<String, BTreeMap<String, f64>> {
        let mut result = BTreeMap::new();

        for (vm, items) in self.data.read().await.iter() {
            let mut vm_result = BTreeMap::new();

            for itm in items.values() {
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

        for (vm, items) in self.data.read().await.iter() {
            let mut vm_result = BTreeMap::new();

            for itm in items.values() {
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

    pub async fn get_vm_cpu_and_mem(&self) -> BTreeMap<String, (f64, i64, usize)> {
        let mut result = BTreeMap::new();

        let read_access = self.data.read().await;

        for (vm, data) in read_access.iter() {
            let mut cpu = 0.0;
            let mut mem = 0;
            let mut amount = 0;

            for itm in data.values() {
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
    ) -> Vec<(Option<String>, ContainerModel)> {
        let read_access = self.data.read().await;

        match selected_vm {
            SelectedVm::All => {
                let mut result = Vec::new();

                for (vm, items) in read_access.iter() {
                    for itm in items.values() {
                        result.push((Some(vm.to_string()), itm.clone()));
                    }
                }

                result
            }
            SelectedVm::SingleVm(vm) => match read_access.get(vm) {
                Some(items) => {
                    let mut result: Vec<(Option<String>, ContainerModel)> =
                        Vec::with_capacity(items.len());

                    for item in items.values() {
                        result.push((None, item.clone()));
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
