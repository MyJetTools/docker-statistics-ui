#![allow(non_snake_case)]

use std::{collections::BTreeMap, time::Duration};

#[cfg(feature = "server")]
use app_ctx::AppCtx;

use dioxus::prelude::*;
use models::{MetricsByVm, VmModel};

mod selected_vm;
mod utils;

#[cfg(feature = "server")]
mod app_ctx;
#[cfg(feature = "server")]
mod background;
#[cfg(feature = "server")]
mod settings;
mod states;

mod models;
mod views;

use serde::*;
use views::*;
#[cfg(feature = "server")]
mod http_client;

use crate::states::*;

#[cfg(feature = "server")]
lazy_static::lazy_static! {
    pub static ref APP_CTX: AppCtx = {
        AppCtx::new()
    };
}

pub const METRICS_HISTORY_SIZE: usize = 150;

fn main() {
    let cfg = dioxus::fullstack::Config::new();

    #[cfg(feature = "server")]
    let cfg = cfg.addr(([0, 0, 0, 0], 9001));

    LaunchBuilder::fullstack().with_cfg(cfg).launch(app)
}

fn app() -> Element {
    use_context_provider(|| Signal::new(MainState::new()));

    use_context_provider(|| Signal::new(DialogState::Hidden));

    let mut started = use_signal(|| false);

    let started_value = *started.read();

    let content = if !started_value {
        rsx! {
            div {

                id: "layout",
                onmounted: move |_| {},
                onmousemove: move |_| {}
            }
        }
    } else {
        rsx! {

            div { id: "layout",

                div { id: "left-panel", left_panel {} }
                div { id: "right-panel", containers_list {} }
                dialog::render_dialog {}
            }
        }
    };

    if !started_value {
        started.set(true);
        read_loop();
    }

    content
}

pub fn read_loop() {
    let mut main_state = consume_context::<Signal<MainState>>();

    /*
       let create_eval = use_eval(cx);

       let eval = create_eval(
           r#"
           console.log("Hello from JavaScript!");

           dioxus.send("");

           setInterval(function(){
               dioxus.send("");
           }, 1000);
           "#,
       )
       .unwrap();
    */

    spawn(async move {
        loop {
            let selected_vm = match main_state.read().get_selected_vm() {
                Some(selected_vm) => selected_vm.to_string(),
                None => "".to_string(),
            };

            let result = get_vm_cpu_and_mem(selected_vm).await;

            match result {
                Ok(result) => {
                    main_state.write().vms_state = Some(result.vms);
                    if let Some(metrics) = result.metrics {
                        main_state.write().set_containers(metrics);
                    }
                }
                Err(err) => {
                    println!("Error on get_vm_cpu_and_mem: {:?}", err);
                }
            }

            let _ = sleep().await;
        }
    });
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestApiModel {
    pub vms: BTreeMap<String, VmModel>,
    pub metrics: Option<Vec<MetricsByVm>>,
}

#[server]
async fn get_vm_cpu_and_mem(selected_vm: String) -> Result<RequestApiModel, ServerFnError> {
    let vms = crate::APP_CTX.metrics_cache.get_vm_cpu_and_mem().await;

    let mut metrics = None;
    if !selected_vm.is_empty() {
        let selected_vm = crate::selected_vm::SelectedVm::from_string(selected_vm);
        let mut result = crate::APP_CTX
            .metrics_cache
            .get_metrics_by_vm(&selected_vm)
            .await;

        let access = crate::APP_CTX.metrics_history.lock().await;
        for result in result.iter_mut() {
            if let Some(wrapper) = access.get(&result.container.id) {
                result.container.cpu_usage_history = Some(wrapper.cpu.get_snapshot());
                result.container.mem_usage_history = Some(wrapper.mem.get_snapshot());
            }
        }

        metrics = Some(result);
    }

    Ok(RequestApiModel { vms, metrics })
}

#[server]
async fn sleep() -> Result<(), ServerFnError> {
    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
