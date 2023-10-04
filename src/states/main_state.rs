use crate::app_ctx::ContainerModel;

use super::SelectedVm;

pub struct MainState {
    pub state_no: usize,
    selected_vm: Option<SelectedVm>,
    containers: Option<Vec<(Option<String>, String, ContainerModel)>>,
    pub filter: String,

    pub dialog_is_shown: bool,
}

impl MainState {
    pub fn new() -> Self {
        Self {
            selected_vm: None,
            containers: None,
            filter: "".to_string(),
            state_no: 0,
            dialog_is_shown: false,
        }
    }

    pub fn set_selected_vm(&mut self, selected_vm: SelectedVm) {
        self.selected_vm = Some(selected_vm);
        self.containers = None;
        self.state_no += 1;
    }

    pub fn is_single_vm_selected(&self, vm: &str) -> bool {
        match self.selected_vm.as_ref() {
            Some(value) => {
                return value.is_single_selected_with_name(vm);
            }
            None => false,
        }
    }

    pub fn is_all_vms_selected(&self) -> bool {
        match self.selected_vm.as_ref() {
            Some(value) => {
                return value.is_all();
            }
            None => false,
        }
    }

    pub fn get_selected_vm(&self) -> Option<SelectedVm> {
        self.selected_vm.clone()
    }

    pub fn get_containers(&self) -> Option<Vec<&(Option<String>, String, ContainerModel)>> {
        let items = self.containers.as_ref()?;

        let mut result = Vec::with_capacity(items.len());
        for itm in items.iter() {
            if itm.2.filter_me(&self.filter) {
                result.push(itm)
            }
        }

        result.sort_by(|a, b| a.2.image.cmp(&b.2.image));

        Some(result)
    }

    pub fn set_containers(&mut self, containers: Vec<(Option<String>, String, ContainerModel)>) {
        self.containers = Some(containers);
    }
}
