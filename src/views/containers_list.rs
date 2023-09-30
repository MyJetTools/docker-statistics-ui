use dioxus::prelude::*;

use super::icons::*;
use crate::{format_mem, states::SelectedVm, APP_CTX};

pub fn containers_list(cx: Scope) -> Element {
    let selected_vm_state = use_shared_state::<SelectedVm>(cx).unwrap();

    let show_disabled_state = use_state(cx, || false);

    match selected_vm_state.read().get_containers() {
        Some(templates) => {
            let templates = templates
                .iter()
                .filter(|itm| {
                    if !show_disabled_state && !itm.enabled {
                        return false;
                    }
                    true
                })
                .map(|itm| {
                    let color = if itm.enabled { "black" } else { "lightgray" };
                    let cpu_usage = if let Some(usage) = itm.cpu.usage {
                        format!("{:.2}%", usage)
                    } else {
                        "N/A".to_string()
                    };

                    let mem_limit = if let Some(usage) = itm.mem.limit {
                        format_mem(usage)
                    } else {
                        "N/A".to_string()
                    };

                    let mem_usage = if let Some(usage) = itm.mem.usage {
                        format_mem(usage)
                    } else {
                        "N/A".to_string()
                    };

                    let items = if let Some(labels) = &itm.labels {
                        let items = labels.iter().map(|(key, value)| {
                            rsx! { div { style: "font-size:10px; padding:0", "{key}={value}" } }
                        });

                        rsx! {items}
                    } else {
                        rsx! { div {} }
                    };
                    rsx! {
                        tr { style: "border-top: 1px solid lightgray; color: {color}",
                            td { "{itm.image}" }
                            td { items }

                            td { cpu_usage }
                            td { "{mem_usage}/{mem_limit}" }
                            td {}
                            td {}
                        }
                    }
                });

            let show_disabled = if *show_disabled_state.get() {
                rsx! {
                    button {
                        style: "width: 110px;",
                        class: "btn btn-sm  btn-danger",
                        onclick: move |_| {
                            show_disabled_state.set(false);
                        },
                        "Hide disabled"
                    }
                }
            } else {
                rsx! {
                    button {
                        style: "width: 110px;",
                        class: "btn btn-sm btn-outline-danger",

                        onclick: move |_| {
                            show_disabled_state.set(true);
                        },
                        "Show disabled"
                    }
                }
            };

            let selected_value = selected_vm_state.read().filter.to_string();
            render! {
                table { class: "table table-striped", style: "text-align: left;",
                    tr {
                        th { colspan: 2,
                            table {
                                tr {
                                    td { "Name" }
                                    td { style: "width:100%",
                                        div { class: "input-group",
                                            span { class: "input-group-text", search_icon {} }
                                            input {
                                                class: "form-control form-control-sm",
                                                value: "{selected_value}",
                                                oninput: move |cx| {
                                                    println!("Setting Filtered value {}", cx.value.to_string());
                                                    selected_vm_state.write().filter = cx.value.to_string();
                                                }
                                            }
                                        }
                                    }
                                    td { show_disabled }
                                }
                            }
                        }
                        th { "Cpu" }
                        th { "Mem" }
                        th {}
                    }

                    templates.into_iter()
                }
            }
        }
        None => {
            load_containers(&cx, &selected_vm_state);
            render! { div {} }
        }
    }
}

fn load_containers(cx: &Scope, selected_vm_state: &UseSharedState<SelectedVm>) {
    let selected_vm = selected_vm_state.read().get_selected_vm();
    if selected_vm.is_none() {
        return;
    }

    let selected_vm = selected_vm.unwrap();

    let selected_vm_state = selected_vm_state.to_owned();

    println!("Started loop for {}", selected_vm);

    cx.spawn(async move {
        let mut no = 0;
        loop {
            let selected_vm = selected_vm.clone();

            if !selected_vm_state.read().is_vm_selected(&selected_vm) {
                println!("Stopped {}", selected_vm);
                break;
            }

            let delay = if no == 0 {
                std::time::Duration::from_millis(100)
            } else {
                std::time::Duration::from_secs(1)
            };

            let items = tokio::spawn(async move {
                tokio::time::sleep(delay).await;
                (
                    APP_CTX.metrics_cache.get_metrics_by_vm(&selected_vm).await,
                    selected_vm,
                )
            })
            .await;

            no += 1;
            let (items, selected_vm) = items.unwrap();

            if selected_vm_state.read().is_vm_selected(&selected_vm) {
                println!("Settings containers for vm {} ", selected_vm);
                selected_vm_state.write().set_containers(items);
            }
        }
    });
}
