use std::collections::BTreeMap;

use dioxus::prelude::*;

use crate::{format_mem, states::SelectedVm, views::icons::*, APP_CTX};

pub fn left_panel(cx: Scope) -> Element {
    let vms_state: &UseState<Option<BTreeMap<String, (f64, i64)>>> = use_state(cx, || None);
    let env_name = use_state(cx, || "".to_string());

    let selected_vm = use_shared_state::<SelectedVm>(cx).unwrap();

    match vms_state.get() {
        Some(vms) => {
            let items = vms.iter().map(|itm| {
                let (vm, (cpu, mem)) = itm;

                let vm_cloned = vm.clone();

                let server_icon = server_icon(cx);
                let mem_icon = memory_icon(cx);
                let cpu_icon = cpu_icon(cx);

                let content = rsx! {
                    table {
                        tr {
                            td { server_icon }
                            td {
                                div { span { style: "font-size:12px", "{vm}" } }
                                div {
                                    cpu_icon,
                                    span { style: "font-size:10px", ":{cpu:.2}%  " }
                                    mem_icon,
                                    span { style: "font-size:10px", ":{format_mem(*mem)}" }
                                }
                            }
                        }
                    }
                };

                if selected_vm.read().is_vm_selected(vm) {
                    rsx! {
                        div { class: "menu-item menu-active", content }
                    }
                } else {
                    rsx! {
                        div {
                            class: "menu-item",
                            onclick: move |_| {
                                selected_vm.write().set_selected_vm(vm_cloned.to_string());
                            },
                            content
                        }
                    }
                }
            });

            return render! {
                h1 { "Dockers" }
                h4 { id: "env-type", "{env_name.get()}" }
                items
            };
        }
        None => {
            read_loop(&cx, vms_state);
            return render! {"Loading..."};
        }
    }
}

fn read_loop(cx: &Scope, vms_state: &UseState<Option<BTreeMap<String, (f64, i64)>>>) {
    let vms_state = vms_state.to_owned();
    cx.spawn(async move {
        let mut no = 0;
        loop {
            let result = if no > 0 {
                tokio::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

                    APP_CTX.metrics_cache.get_vm_cpu_and_mem().await
                })
                .await
            } else {
                tokio::spawn(async move { APP_CTX.metrics_cache.get_vm_cpu_and_mem().await }).await
            };

            no += 1;

            match result {
                Ok(result) => {
                    vms_state.set(Some(result));
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                }
            }
        }
    })
}
