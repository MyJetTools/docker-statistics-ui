use std::collections::BTreeMap;

use dioxus::prelude::{SvgAttributes, *};

use crate::{
    format_mem,
    states::{MainState, SelectedVm},
    views::icons::*,
    APP_CTX,
};

pub fn left_panel(cx: Scope) -> Element {
    let vms_state: &UseState<Option<BTreeMap<String, (f64, i64, usize)>>> = use_state(cx, || None);
    let env_name = use_state(cx, || "".to_string());

    let selected_vm_state = use_shared_state::<MainState>(cx).unwrap();

    let selected_vm = selected_vm_state.read();

    let mut total_cpu = 0.0;
    let mut total_mem = 0;
    let mut total_amount = 0;

    match vms_state.get() {
        Some(vms) => {
            let items = vms.into_iter().map(|itm| {
                let (vm, (cpu, mem, amount)) = itm;

                total_cpu += cpu;
                total_mem += mem;
                total_amount += amount;

                if selected_vm.is_single_vm_selected(vm) {
                    rsx! {
                        div { class: "menu-item menu-active",
                            render_vm_menu_item { name: vm, cpu: *cpu, mem: *mem, amount: *amount }
                        }
                    }
                } else {
                    rsx! {
                        div {
                            class: "menu-item",
                            onclick: move |_| {
                                selected_vm_state.write().set_selected_vm(SelectedVm::SingleVm(vm.to_string()));
                            },
                            render_vm_menu_item { name: vm, cpu: *cpu, mem: *mem, amount: *amount }
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
                    render_vm_menu_item { name: "All VMs", cpu: total_cpu, mem: total_mem, amount: total_amount }
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

fn read_loop(cx: &Scope, vms_state: &UseState<Option<BTreeMap<String, (f64, i64, usize)>>>) {
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
    cx: Scope<'s>,
    name: &'s str,
    cpu: f64,
    mem: i64,
    amount: usize,
) -> Element {
    let mem = format_mem(*mem);
    render! {
        table {
            tr {
                td { server_icon {} }
                td {
                    div { span { style: "font-size:12px", "{name}: ({amount})" } }
                    div {
                        cpu_icon {}
                        span { font: "courier", style: "font-size:10px", ":{cpu:.3}  " }
                        memory_icon {}
                        span { style: "font-size:10px", ":{mem}" }
                    }
                }
            }
        }
    }
}
