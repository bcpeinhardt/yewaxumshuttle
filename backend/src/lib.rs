use axum::{routing::get, Router};
use sync_wrapper::SyncWrapper;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    let router = Router::new()
        .route("/hello", get(hello_world))
        .merge(axum_extra::routing::SpaRouter::new("/assets", "dist"));
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
