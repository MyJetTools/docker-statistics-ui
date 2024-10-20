mod app_ctx;
mod background;

pub mod http_client;
mod settings;

mod ssh_certs_cache;
mod ssh_settings;

lazy_static::lazy_static! {
    pub static ref APP_CTX: app_ctx::AppCtx = {
        app_ctx::AppCtx::new()
    };
}
