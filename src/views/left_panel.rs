use std::collections::BTreeMap;

use dioxus::prelude::*;

use crate::{
    app_ctx::VmModel,
    format_mem,
    states::{MainState, SelectedVm},
    views::icons::*,
    APP_CTX,
};

pub fn left_panel(cx: Scope) -> Element {
    let vms_state: &UseState<Option<BTreeMap<String, VmModel>>> = use_state(cx, || None);
    let env_name = use_state(cx, || "".to_string());

    let selected_vm_state = use_shared_state::<MainState>(cx).unwrap();

    let selected_vm = selected_vm_state.read();

    let mut all_vms = VmModel {
        api_url: "".to_string(),
        cpu: 0.0,
        mem: 0,
        mem_limit: 0,
        containers_amount: 0,
    };

    match vms_state.get() {
        Some(vms) => {
            let items = vms.into_iter().map(|itm| {
                let (vm, vm_model) = itm;

                all_vms.cpu += vm_model.cpu;
                all_vms.mem += vm_model.mem;
                all_vms.containers_amount += vm_model.containers_amount;
                all_vms.mem_limit += vm_model.mem_limit;

                if selected_vm.is_single_vm_selected(vm) {
                    rsx! {
                        div { class: "menu-item menu-active",
                            render_vm_menu_item {
                                name: vm,
                                cpu: vm_model.cpu,
                                mem: vm_model.mem,
                                mem_limit: vm_model.mem_limit,
                                amount: vm_model.containers_amount,
                                url: vm_model.api_url.to_string()
                            }
                        }
                    }
                } else {
                    rsx! {
                        div {
                            class: "menu-item",
                            onclick: move |_| {
                                selected_vm_state.write().set_selected_vm(SelectedVm::SingleVm(vm.to_string()));
                            },
                            render_vm_menu_item {
                                name: vm,
                                cpu: vm_model.cpu,
                                mem: vm_model.mem,
                                mem_limit: vm_model.mem_limit,
                                amount: vm_model.containers_amount,
                                url: vm_model.api_url.to_string()
                            }
                        }
                    }
                }
            });

            let mut items: Vec<_> = items.collect();

            items.push(rsx! { hr {} });

            let menu_active = if selected_vm.is_all_vms_selected() {
                "menu-active"
            } else {
                ""
            };

            items.push(rsx! {
                div {
                    class: "menu-item {menu_active}",
                    onclick: move |_| {
                        selected_vm_state.write().set_selected_vm(SelectedVm::All);
                    },
                    render_vm_menu_item {
                        name: "All VMs",
                        cpu: all_vms.cpu,
                        mem: all_vms.mem,
                        mem_limit: all_vms.mem_limit,
                        amount: all_vms.containers_amount,
                        url: all_vms.api_url.to_string()
                    }
                }
            });

            return render! {
                h1 { "Dockers" }
                h4 { id: "env-type", "{env_name.get()}" }
                items.into_iter()
            };
        }
        None => {
            read_loop(&cx, vms_state);
            return render! {"Loading..."};
        }
    }
}

fn read_loop(cx: &Scope, vms_state: &UseState<Option<BTreeMap<String, VmModel>>>) {
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

#[inline_props]
fn render_vm_menu_item<'s>(
    cx: Scope,
    name: &'s str,
    cpu: f64,
    mem: i64,
    mem_limit: i64,
    amount: usize,
    url: String,
) -> Element {
    let mem = format_mem(mem);
    let mem_limit = format_mem(mem_limit);
    render! {
        table {
            tr { title: "{url}",
                td { server_icon {} }
                td {
                    div { span { style: "font-size:12px", "{name}: ({amount})" } }
                    div {
                        cpu_icon {}
                        span { font: "courier", style: "font-size:10px", ":{cpu:.3}  " }
                        memory_icon {}
                        span { style: "font-size:10px", ":{mem}/{mem_limit}" }
                    }
                }
            }
        }
    }
}
