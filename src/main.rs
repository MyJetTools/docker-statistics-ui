#![allow(non_snake_case)]

#[cfg(feature = "ssr")]
use app_ctx::AppCtx;

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use router::AppRoute;

mod router;
mod selected_vm;

#[cfg(feature = "ssr")]
mod app_ctx;
#[cfg(feature = "ssr")]
mod background;
#[cfg(feature = "ssr")]
mod settings;
mod states;

mod models;
mod views;

use views::*;
#[cfg(feature = "ssr")]
mod http_client;

use crate::states::*;

#[cfg(feature = "ssr")]
lazy_static::lazy_static! {
    pub static ref APP_CTX: AppCtx = {
        AppCtx::new()
    };
}

pub const METRICS_HISTORY_SIZE: usize = 150;

fn main() {
    let config = LaunchBuilder::<FullstackRouterConfig<AppRoute>>::router();

    #[cfg(feature = "ssr")]
    let config = config.addr(std::net::SocketAddr::from(([0, 0, 0, 0], 8080)));

    config.launch();
}

fn Home(cx: Scope) -> Element {
    use_shared_state_provider(cx, || MainState::new());
    use_shared_state_provider(cx, || DialogState::Hidden);

    render! {
        div { id: "layout",
            div { id: "left-panel", left_panel {} }
            div { id: "right-panel", containers_list {} }
            dialog::render_dialog {}
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
