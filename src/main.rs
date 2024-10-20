#![allow(non_snake_case)]

use std::collections::BTreeMap;

#[cfg(feature = "server")]
mod server;

mod models;

use dioxus::prelude::*;
use models::*;

mod selected_vm;
mod utils;

mod states;

mod views;

use serde::*;
use views::*;

use crate::states::*;

pub const METRICS_HISTORY_SIZE: usize = 150;

fn main() {
    let cfg = dioxus::fullstack::Config::new();

    #[cfg(feature = "server")]
    let cfg = cfg.addr(([0, 0, 0, 0], 9001));

    LaunchBuilder::fullstack().with_cfg(cfg).launch(app)
}

#[component]
fn app() -> Element {
    use_context_provider(|| Signal::new(MainState::new()));
    use_context_provider(|| Signal::new(DialogState::Hidden));

    let mut main_state = consume_context::<Signal<MainState>>();

    let has_envs = { main_state.read().has_envs() };

    if has_envs {
        return rsx! {
            ActiveApp {}
        };
    }

    let resource = use_resource(|| get_envs());

    let data = resource.read_unchecked();

    match &*data {
        Some(data) => match data {
            Ok(result) => {
                main_state.write().set_environments(result.clone());
                return rsx! {
                    ActiveApp {}
                };
            }
            Err(err) => {
                let err = format!("Error loading environments. Err: {}", err);
                return rsx! {
                    {err}
                };
            }
        },

        None => {
            return rsx! { "Loading environments..." };
        }
    }
}

#[component]
fn ActiveApp() -> Element {
    let main_state = consume_context::<Signal<MainState>>();
    let mut started = use_signal(|| false);

    let started_value = { *started.read() };

    let env = { main_state.read().selected_env.clone() };

    if !started_value {
        started.set(true);
        read_loop(main_state);
    }

    rsx! {

        div { id: "layout",
            div { id: "left-panel", left_panel {} }
            div { id: "right-panel",
                containers_list { env }
            }
            dialog::render_dialog {}
        }
    }
}

pub fn read_loop(mut main_state: Signal<MainState>) {
    let mut eval = eval(
        r#"

        dioxus.send("");
        
        setInterval(function(){
            dioxus.send("");
        }, 1000);
        "#,
    );

    spawn(async move {
        loop {
            eval.recv().await.unwrap();

            let (env, selected_vm) = { main_state.read().get_selected_vm() };

            let selected_vm = match selected_vm {
                Some(value) => value.to_string(),
                None => "".to_string(),
            };

            let result = get_vm_cpu_and_mem(env, selected_vm).await;

            match result {
                Ok(result) => {
                    let mut write_state = main_state.write();
                    write_state.vms_state = Some(result.vms);
                    if let Some(metrics) = result.metrics {
                        write_state.set_containers(metrics);
                    }
                }
                Err(err) => {
                    println!("Error on get_vm_cpu_and_mem: {:?}", err);
                }
            }
        }
    });
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestApiModel {
    pub vms: BTreeMap<String, VmModel>,
    pub metrics: Option<Vec<MetricsByVm>>,
}

#[server]
async fn get_envs() -> Result<Vec<String>, ServerFnError> {
    let settings = crate::server::APP_CTX.settings_reader.get_settings().await;
    Ok(settings.envs.keys().cloned().collect())
}

#[server]
async fn get_vm_cpu_and_mem(
    env: String,
    selected_vm: String,
) -> Result<RequestApiModel, ServerFnError> {
    let cache_access_by_env = crate::server::APP_CTX.data_cache_by_env.lock().await;

    let cache_access = cache_access_by_env.envs.get(&env);

    if cache_access.is_none() {
        return Ok(RequestApiModel {
            vms: BTreeMap::new(),
            metrics: None,
        });
    }

    let cache_access = cache_access.unwrap();

    let vms = cache_access.get_vm_cpu_and_mem();

    let mut metrics = None;
    if !selected_vm.is_empty() {
        let selected_vm = crate::selected_vm::SelectedVm::from_string(selected_vm);
        let mut result = cache_access.get_metrics_by_vm(&selected_vm);

        for result in result.iter_mut() {
            if let Some(wrapper) = cache_access.metrics_history.get(&result.container.id) {
                result.container.cpu_usage_history = Some(wrapper.cpu.get_snapshot());
                result.container.mem_usage_history = Some(wrapper.mem.get_snapshot());
            }
        }

        metrics = Some(result);
    }

    Ok(RequestApiModel { vms, metrics })
}
