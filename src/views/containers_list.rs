use dioxus::prelude::*;
use super::icons::*;
use crate::{
    states::{MainState, DialogState, DialogType},
    views::{render_cpu_graph, render_mem_graph}, utils::format_mem,
};

pub fn containers_list(cx: Scope) -> Element {
    let main_state = use_shared_state::<MainState>(cx).unwrap();


    let dialog_sate = use_shared_state::<DialogState>(cx).unwrap();

    let show_disabled_state = use_state(cx, || false);


    /*
    let loop_is_run = use_state(cx, || false);


    if !loop_is_run.get() && main_state.read().get_selected_vm().is_some(){
        loop_is_run.set(true);
        load_containers(&cx, &main_state);
    }
 */
    match main_state.read().get_containers() {
        Some(containers) => {
            let containers = containers
                .iter()
                .filter(|itm| {
                    if !show_disabled_state && !itm.container.enabled {
                        return false;
                    }
                    true
                })
                .map(|itm| {
                    let color = if itm.container.enabled { "black" } else { "lightgray" };
                    let cpu_usage = if let Some(usage) = itm.container.cpu.usage {
                        format!("{:.3}", usage)
                    } else {
                        "N/A".to_string()
                    };

                    let mem_limit = if let Some(usage) = itm.container.mem.limit {
                        format_mem(usage)
                    } else {
                        "N/A".to_string()
                    };

                    let mem_usage = if let Some(usage) = itm.container.mem.usage {
                        format_mem(usage)
                    } else {
                        "N/A".to_string()
                    };

                    let id_cloned = itm.container.id.clone();


                    let url_show_logs = itm.url.clone();

                    let vm_name = if let Some(vm_name) = &itm.vm {
                        rsx! {
                            div { title: "{vm_name}",
                                server_icon_16 {}
                                span { "{itm.url}" }
                            }
                        }   
                    } else {
                        rsx! { div {} }
                    };


       
                    let (cpu_graph, mem_graph) = if let Some(cpu_snapshot) = &itm.container.cpu_usage_history {


                        let mem_limit = if let Some(usage) = itm.container.mem.limit {
                            usage
                        } else {
                            0
                        };

                        let mem_snapshot = itm.container.mem_usage_history.as_ref().unwrap();
                        (
                            rsx! { render_cpu_graph{values: cpu_snapshot.clone()  }   },
                            rsx! {div{render_mem_graph{values: mem_snapshot.clone(), mem_limit:mem_limit  } }},
                        )
                    } else {
                        (rsx! {div{}}, rsx! {div{}})
                    };

                    let items = if let Some(labels) = &itm.container.labels {
                        let items = labels.iter().map(|(key, value)| {
                            rsx! { div { style: "font-size:10px; padding:0", "{key}={value}" } }
                        });

                        rsx! {items}
                    } else {
                        rsx! { div {} }
                    };

                    let image_cloned = itm.container.image.clone();
                    rsx! {
                        tr { style: "border-top: 1px solid lightgray; color: {color}",
                            td {
                                div { "{itm.container.image}" }
                                div { vm_name }
                                div {
                                    button {
                                        class: "btn btn-sm btn-primary",
                                        onclick: move |_| {
                                            dialog_sate
                                                .write()
                                                .show_dialog(
                                                    format!("Logs of container {}", image_cloned),
                                                    DialogType::ShowLogs {
                                                        container_id: id_cloned.clone(),
                                                        url: url_show_logs.clone(),
                                                    },
                                                );
                                        },
                                        "Show logs"
                                    }
                                }
                            }
                            td { items }

                            td {
                                div { style: "cursor:pointer; padding:0", cpu_icon(cx), ": {cpu_usage}" }
                                div { style: "padding:0", cpu_graph }
                                div { style: "cursor:pointer;padding:0;font-size: 12px; margin-top: 5px;",
                                    memory_icon(cx),
                                    ": {mem_usage}/{mem_limit}"
                                }
                                div { style: "padding:0", mem_graph }
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

            render! { div {} }
        }
    }
}

/*
fn load_containers(cx: &Scope, main_state: &UseSharedState<MainState>) {


    let main_state = main_state.to_owned();




    cx.spawn(async move {
      let mut no = 0;
      let mut loop_state_no = main_state.read().state_no;
      loop {
            let selected_vm = main_state.read().get_selected_vm().unwrap();

            if loop_state_no != main_state.read().state_no {
                no = 0;
                loop_state_no = main_state.read().state_no;
            }

            if let Ok(items) = get_metrics_by_vm(selected_vm.to_string(), no>0).await{
    
                if main_state.read().state_no == loop_state_no {
                    main_state.write().set_containers(items);
                }
            }

            no += 1;

           // tokio::time::sleep(delay).await;
       }
    });
}


#[server]
async fn get_metrics_by_vm(selected_vm: String, background:bool) -> Result<Vec<MetricsByVm>, ServerFnError>{
    if background{
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
    
    let selected_vm = crate::selected_vm::SelectedVm::from_string(selected_vm);
    let mut result = crate::APP_CTX.metrics_cache.get_metrics_by_vm(&selected_vm).await;

    let access = crate::APP_CTX.metrics_history.lock().await;
    for result in result.iter_mut(){
        if let Some(wrapper) = access.get(&result.container.id){
            result.container.cpu_usage_history = Some(wrapper.cpu.get_snapshot());
            result.container.mem_usage_history = Some(wrapper.mem.get_snapshot());
        }
    }

    Ok(result)
} */