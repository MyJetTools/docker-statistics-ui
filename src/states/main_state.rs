use std::sync::Arc;

use crate::http_client::ContainerJsonModel;

pub struct SelectedVm {
    value: Option<String>,
    containers: Option<Arc<Vec<ContainerJsonModel>>>,
    pub filter: String,
}

impl SelectedVm {
    pub fn new() -> Self {
        Self {
            value: None,
            containers: None,
            filter: "".to_string(),
        }
    }

    pub fn set_selected_vm(&mut self, vm: String) {
        self.value = Some(vm);
        self.containers = None;
    }

    pub fn is_vm_selected(&self, vm: &str) -> bool {
        match self.value.as_ref() {
            Some(value) => value == vm,
            None => false,
        }
    }

    pub fn get_selected_vm(&self) -> Option<String> {
        self.value.clone()
    }

    pub fn get_containers(&self) -> Option<Vec<&ContainerJsonModel>> {
        let items = self.containers.as_ref()?;

        let mut result = Vec::with_capacity(items.len());
        for itm in items.iter() {
            if itm.filter_me(&self.filter) {
                result.push(itm)
            }
        }

        Some(result)
    }

    pub fn set_containers(&mut self, containers: Arc<Vec<ContainerJsonModel>>) {
        self.containers = Some(containers);
    }
}
