use std::{sync::Arc, time::Duration};

use app_ctx::AppCtx;
use background::UpdateMetricsCacheTimer;
use dioxus::prelude::*;
use dioxus_liveview::LiveViewPool;
use rust_extensions::MyTimer;
use salvo::prelude::*;

mod app_ctx;
mod background;
mod http_server;
mod settings;
mod states;
mod static_resources;
mod views;

use settings::SettingsReader;
use views::*;
mod http_client;

use crate::states::*;

lazy_static::lazy_static! {
    pub static ref APP_CTX: AppCtx = {
        AppCtx::new()
    };
}

#[tokio::main]
async fn main() {
    let settings_reader = crate::settings::SettingsReader::new(".docker-statistics-ui").await;
    let settings_reader: Arc<SettingsReader> = Arc::new(settings_reader);
    APP_CTX.inject_settings(settings_reader).await;

    let mut timer_5s = MyTimer::new(Duration::from_secs(6));
    timer_5s.register_timer("MetricsUpdate", Arc::new(UpdateMetricsCacheTimer));
    timer_5s.start(APP_CTX.app_states.clone(), my_logger::LOGGER.clone());

    let acceptor = TcpListener::new("0.0.0.0:9001").bind().await;
    let view = LiveViewPool::new();

    let router = Router::new()
        .hoop(affix::inject(Arc::new(view)))
        .get(http_server::index)
        .push(Router::with_path("ws").get(http_server::connect))
        .push(Router::with_path("img/<**path>").get(StaticDir::new("./files/img")));

    Server::new(acceptor).serve(router).await;
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || SelectedVm::new());
    use_shared_state_provider(cx, || DialogState::Hidden);

    render! {
        div { id: "layout",
            div { id: "left-panel", left_panel {} }
            div { id: "right-panel", containers_list {} }
        }
    }
}

pub fn format_mem(mem: i64) -> String {
    let mem = mem as f64;
    if mem < 1024.0 {
        return format!("{:.2}B", mem);
    }

    let mem = mem / 1024.0;

    if mem < 1024.0 {
        return format!("{:.2}KB", mem);
    }

    let mem = mem / 1024.0;

    if mem < 1024.0 {
        return format!("{:.2}MB", mem);
    }

    let mem = mem / 1024.0;

    return format!("{:.2}GB", mem);
}
