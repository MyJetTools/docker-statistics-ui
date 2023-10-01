use dioxus::prelude::*;

use super::icons::*;
use crate::{format_mem, states::MainState, APP_CTX, views::{render_mem_graph, render_cpu_graph}};

pub fn containers_list(cx: Scope) -> Element {
    let main_state = use_shared_state::<MainState>(cx).unwrap();

    let show_disabled_state = use_state(cx, || false);

    match main_state.read().get_containers() {
        Some(containers) => {
            let containers = containers
                .iter()
                .filter(|(_, itm)| {
                    if !show_disabled_state && !itm.enabled {
                        return false;
                    }
                    true
                })
                
                .map(|(vm_name, itm)| {
                    let color = if itm.enabled { "black" } else { "lightgray" };
                    let cpu_usage = if let Some(usage) = itm.cpu.usage {
                        format!("{:.3}", usage)
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

                    let vm_name = if let Some(vm_name) = vm_name{
                        rsx!{
                            server_icon_16 {}
                            span { "{vm_name}" }
                        }
                    }else{
                        rsx!{ div {} }
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
                            td {
                                div { "{itm.image}" }
                                div { vm_name }
                            }
                            td { items }

                            td {
                                div { cpu_icon(cx), ": {cpu_usage}" }
                                div { render_cpu_graph { values: itm.cpu_usage_history.get_snapshot() } }
                                div { style: "font-size: 12px", memory_icon(cx), ": {mem_usage}/{mem_limit}" }
                                div { render_mem_graph { values: itm.mem_usage_history.get_snapshot() } }
                            }
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

            let selected_value = main_state.read().filter.to_string();



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
                                                    main_state.write().filter = cx.value.to_string();
                                                }
                                            }
                                        }
                                    }
                                    td { show_disabled }
                                }
                            }
                        }
                        th { "Cpu/Mem" }
                    }

                    containers.into_iter()
                }
            }
        }
        None => {
            load_containers(&cx, &main_state);
            render! { div {} }
        }
    }
}

fn load_containers(cx: &Scope, main_state: &UseSharedState<MainState>) {
    let selected_vm = main_state.read().get_selected_vm();
    if selected_vm.is_none() {
        return;
    }

    let selected_vm = selected_vm.unwrap();

    let main_state = main_state.to_owned();

    let loop_state_no = main_state.read().state_no;

    cx.spawn(async move {
        let mut no = 0;
        loop {
            let selected_vm = selected_vm.clone();

            if !main_state.read().state_no == loop_state_no {
                break;
            }

            let delay = if no == 0 {
                std::time::Duration::from_millis(100)
            } else {
                std::time::Duration::from_secs(1)
            };

            let items = tokio::spawn(async move {
                tokio::time::sleep(delay).await;
                APP_CTX.metrics_cache.get_metrics_by_vm(&selected_vm).await
            })
            .await;

            no += 1;
            let items = items.unwrap();

            if main_state.read().state_no == loop_state_no {
                main_state.write().set_containers(items);
            }
        }
    });
}
