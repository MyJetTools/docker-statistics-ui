use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
#[derive(Props, PartialEq, Eq)]
pub struct ShowPopulatedYamlProps {
    pub url: String,
    pub container_id: String,
}

pub fn show_logs<'s>(cx: Scope<'s, ShowPopulatedYamlProps>) -> Element {
    let logs = use_state(cx, || "".to_string());
    let lines_amount = use_state(cx, || 100u32);

    if logs.is_empty() {
        let url = cx.props.url.to_string();
        let id = cx.props.container_id.to_string();
        let lines_amount = *lines_amount.get();
        let logs = logs.to_owned();

        cx.spawn(async move {
            let result = get_logs(url, id, lines_amount).await;

            match result {
                Ok(result) => logs.set(result),
                Err(err) => logs.set(format!("Error during receiving logs: {:?}", err)),
            }
        });

        return render! {
            div { class: "modal-content", div { class: "form-control modal-content-full-screen", "Loading..." } }
        };
    }

    let amount_value = lines_amount.get().to_string();

    render! {
        div { class: "modal-content",
            div { class: "input-group",
                span { class: "input-group-text", "Amount" }
                input {
                    class: "form-control",
                    value: "{amount_value}",
                    r#type: "number",
                    onchange: move |cx| {
                        lines_amount.set(cx.value.parse().unwrap_or(100));
                    }
                }
                button {
                    class: "btn btn-outline-secondary",
                    onclick: move |_| {
                        let url = cx.props.url.to_string();
                        let id = cx.props.container_id.to_string();
                        let lines_amount = *lines_amount.get();
                        let logs = logs.to_owned();
                        cx.spawn(async move {
                            let result = get_logs(url, id, lines_amount).await;
                            match result {
                                Ok(result) => logs.set(result),
                                Err(err) => logs.set(format!("Error during receiving logs: {:?}", err)),
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
                logs.as_str()
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
async fn get_logs(url: String, id: String, lines_amount: u32) -> Result<String, ServerFnError> {
    let result = crate::http_client::get_logs(url, id, lines_amount).await;
    let result = match result {
        Ok(result) => result,
        Err(err) => format!("Error during receiving logs: {:?}", err),
    };

    if result.len() == 0 {
        return Ok("No Logs Received".to_string());
    }

    Ok(result)
}
