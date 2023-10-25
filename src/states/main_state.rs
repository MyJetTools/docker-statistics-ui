use crate::{models::MetricsByVm, selected_vm::SelectedVm};

pub struct MainState {
    pub state_no: usize,
    pub data_request_no: i32,
    selected_vm: Option<SelectedVm>,
    containers: Option<Vec<MetricsByVm>>,
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
            data_request_no: 0,
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

    pub fn get_containers(&self) -> Option<Vec<&MetricsByVm>> {
        let items = self.containers.as_ref()?;

        let mut result = Vec::with_capacity(items.len());
        for itm in items.iter() {
            if itm.container.filter_me(&self.filter) {
                result.push(itm)
            }
        }

        result.sort_by(|a, b| a.container.image.cmp(&b.container.image));

        Some(result)
    }

    pub fn set_containers(&mut self, containers: Vec<MetricsByVm>) {
        self.containers = Some(containers);
    }
}
