#![allow(non_snake_case)]

use std::collections::BTreeMap;

#[cfg(feature = "ssr")]
use app_ctx::AppCtx;

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use models::{MetricsByVm, VmModel};
use router::AppRoute;

mod router;
mod selected_vm;
mod utils;

#[cfg(feature = "ssr")]
mod app_ctx;
#[cfg(feature = "ssr")]
mod background;
#[cfg(feature = "ssr")]
mod settings;
mod states;

mod models;
mod views;

use serde::*;
use views::*;
#[cfg(feature = "ssr")]
mod http_client;

use crate::states::*;

#[cfg(feature = "ssr")]
lazy_static::lazy_static! {
    pub static ref APP_CTX: AppCtx = {
        AppCtx::new()
    };
}

pub const METRICS_HISTORY_SIZE: usize = 150;

fn main() {
    let config = LaunchBuilder::<FullstackRouterConfig<AppRoute>>::router();

    #[cfg(feature = "ssr")]
    let config = config.addr(std::net::SocketAddr::from(([0, 0, 0, 0], 8080)));

    config.launch();
}

fn Home(cx: Scope) -> Element {
    use_shared_state_provider(cx, || MainState::new());
    use_shared_state_provider(cx, || DialogState::Hidden);
    let started = use_state(cx, || false);

    let content = if !started.get() {
        rsx! {
            div {
                id: "layout",
                onmounted: move |_| {
                    if !started.get() {
                        started.set(true);
                        read_loop(&cx);
                    }
                },
                onmousemove: move |_| {
                    if !started.get() {
                        started.set(true);
                        read_loop(&cx);
                    }
                }
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

    render! {content}
}

pub fn read_loop(cx: &Scope) {
    let main_state = use_shared_state::<MainState>(cx).unwrap().to_owned();

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

    cx.spawn(async move {
        loop {
            eval.recv().await.unwrap();

            let selected_vm = match main_state.read().get_selected_vm() {
                Some(selected_vm) => selected_vm.to_string(),
                None => "".to_string(),
            };

            let result = get_vm_cpu_and_mem(selected_vm).await;

            match result {
                Ok(result) => {
                    let mut write_access = main_state.write();
                    write_access.vms_state = Some(result.vms);
                    if let Some(metrics) = result.metrics {
                        write_access.set_containers(metrics);
                    }
                }
                Err(err) => {
                    println!("Error on get_vm_cpu_and_mem: {:?}", err);
                }
            }
        }
    })
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
