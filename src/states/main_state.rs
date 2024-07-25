use std::{collections::BTreeMap, rc::Rc};

use crate::{
    models::{MetricsByVm, VmModel},
    selected_vm::SelectedVm,
};

pub struct MainState {
    pub selected_env: Rc<String>,
    pub envs: Option<Vec<Rc<String>>>,
    pub vms_state: Option<BTreeMap<String, VmModel>>,
    pub state_no: usize,
    pub data_request_no: i32,
    selected_vm: Option<SelectedVm>,
    containers: Option<Vec<MetricsByVm>>,
    filter: String,

    pub dialog_is_shown: bool,
}

impl MainState {
    pub fn new() -> Self {
        Self {
            selected_vm: None,
            envs: None,
            containers: None,
            filter: "".to_string(),
            state_no: 0,
            dialog_is_shown: false,
            data_request_no: 0,
            vms_state: None,
            selected_env: Rc::new("".to_string()),
        }
    }

    pub fn has_envs(&self) -> bool {
        self.envs.is_some()
    }

    pub fn set_active_env(&mut self, env: &str) {
        let found_value = self
            .envs
            .as_ref()
            .unwrap()
            .into_iter()
            .find(|itm| itm.as_str() == env);

        if let Some(found_value) = found_value {
            self.selected_env = found_value.clone();
            self.containers = None;
            self.vms_state = None;
            self.selected_vm = None;
        }
    }

    pub fn set_environments(&mut self, envs: Vec<String>) {
        let envs: Vec<Rc<String>> = envs.into_iter().map(Rc::new).collect();
        self.selected_env = envs[0].clone();
        self.envs = Some(envs);
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

    pub fn get_selected_vm(&self) -> (String, Option<SelectedVm>) {
        (self.selected_env.to_string(), self.selected_vm.clone())
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

    pub fn set_filter(&mut self, value: String) {
        self.filter = value;
    }

    pub fn get_filter(&self) -> &str {
        &self.filter
    }
}
