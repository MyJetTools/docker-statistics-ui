use dioxus::prelude::*;

pub struct LogsValue(String);

impl LogsValue {
    pub fn new() -> Self {
        LogsValue("".to_string())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn set(&mut self, value: String) {
        self.0 = value;
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[component]
pub fn show_logs(env: String, url: String, container_id: String) -> Element {
    let logs = use_signal(|| LogsValue::new());
    let mut lines_amount = use_signal(|| 100u32);

    let logs_read_access = logs.read();

    let lines_amount_value = *lines_amount.read();

    if logs_read_access.is_empty() {
        let url = url.to_string();
        let id = container_id.to_string();
        let env = env.to_string();

        let mut logs_spawned = logs.to_owned();

        spawn(async move {
            let result = get_logs(env, url, id, lines_amount_value).await;

            match result {
                Ok(result) => {
                    logs_spawned.write().set(result);
                }
                Err(err) => logs_spawned
                    .write()
                    .set(format!("Error during receiving logs: {:?}", err)),
            }
        });

        return rsx! {
            div { class: "modal-content",
                div { class: "form-control modal-content-full-screen", "Loading..." }
            }
        };
    }

    let amount_value = lines_amount.to_string();

    rsx! {
        div { class: "modal-content",
            div { class: "input-group",
                span { class: "input-group-text", "Amount" }
                input {
                    class: "form-control",
                    value: "{amount_value}",
                    r#type: "number",
                    onchange: move |cx| {
                        lines_amount.set(cx.value().parse().unwrap_or(100));
                    }
                }
                button {
                    class: "btn btn-outline-secondary",
                    onclick: move |_| {
                        let url = url.to_string();
                        let id = container_id.to_string();
                        let env = env.to_string();
                        let mut logs = logs.to_owned();
                        spawn(async move {
                            let result = get_logs(env, url, id, lines_amount_value).await;
                            match result {
                                Ok(result) => logs.write().set(result),
                                Err(err) => {
                                    logs.write().set(format!("Error during receiving logs: {:?}", err))
                                }
                            }
                        });
                    },
                    "Request"
                }
            }

            textarea {
                style: "height:80vh; font-size: 14px; margin-top:10px",
                class: "form-control modal-content-full-screen",
                readonly: true,
                {logs_read_access.as_str()}
            }
        }
    }
}

/*
fn load_logs<'s>(
    cx: &'s Scope<'s, ShowPopulatedYamlProps>,
    state: &UseState<String>,
    url: String,
    container_id: String,
    lines_amount: u32,
) {
    let state = state.to_owned();

    cx.spawn(async move {
        let mut lines = tokio::spawn(async move {
            let result = get_logs(url, container_id, lines_amount).await;

            match result {
                Ok(result) => result,
                Err(err) => format!("Error during receiving logs: {:?}", err),
            }
        })
        .await
        .unwrap();

        if lines.len() == 0 {
            lines = "No Logs Received".to_string();
        }

        state.set(lines);
    });
}
 */

#[server]
async fn get_logs(
    env: String,
    url: String,
    id: String,
    lines_amount: u32,
) -> Result<String, ServerFnError> {
    let (_, fl_url) = crate::APP_CTX
        .settings_reader
        .get_settings()
        .await
        .get_fl_url(env.as_str(), url.as_str())
        .await;
    let result = crate::http_client::get_logs(fl_url, id, lines_amount).await;
    let result = match result {
        Ok(result) => result,
        Err(err) => format!("Error during receiving logs: {:?}", err),
    };

    if result.len() == 0 {
        return Ok("No Logs Received".to_string());
    }

    Ok(result)
}
