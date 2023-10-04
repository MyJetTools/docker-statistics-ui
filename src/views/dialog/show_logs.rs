use dioxus::prelude::*;

#[derive(Props, PartialEq, Eq)]
pub struct ShowPopulatedYamlProps {
    pub url: String,
    pub container_id: String,
}

pub fn show_logs<'s>(cx: Scope<'s, ShowPopulatedYamlProps>) -> Element {
    let logs = use_state(cx, || "".to_string());
    let lines_amount = use_state(cx, || 100u32);

    if logs.is_empty() {
        load_logs(
            &cx,
            logs,
            cx.props.url.to_string(),
            cx.props.container_id.to_string(),
            *lines_amount.get(),
        );

        return render! {
            div { class: "modal-content", div { class: "form-control modal-content-full-screen", "Loading..." } }
        };
    }

    let amount_value = lines_amount.get().to_string();

    render! {
        div { class: "modal-content",
            span { "Lines amount" }
            input {
                value: "{amount_value}",
                r#type: "number",
                onchange: move |cx| {
                    lines_amount.set(cx.value.parse().unwrap_or(100));
                }
            }
            button {
                class: "btn btn-success, btn-sm",
                onclick: move |_| {
                    load_logs(
                        &cx,
                        logs,
                        cx.props.url.to_string(),
                        cx.props.container_id.to_string(),
                        *lines_amount.get(),
                    );
                },
                "Request"
            }
            textarea {
                style: "height:80vh; font-size: 14px;",
                class: "form-control modal-content-full-screen",
                readonly: true,
                logs.as_str()
            }
        }
    }
}

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
            let result = crate::http_client::get_logs(url, container_id, lines_amount).await;

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
