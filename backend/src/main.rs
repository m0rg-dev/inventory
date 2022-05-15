use std::sync::Arc;

use api::*;
use axum::routing::delete;
use axum::routing::get_service;
use axum::routing::post;
use axum::routing::put;
use axum::{routing::get, Extension, Router};
use hyper::StatusCode;
use model::Database;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

mod api;
mod model;

pub struct State {
    db: Database,
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(State {
        db: Default::default(),
    });

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    let fmt_layer = tracing_subscriber::fmt::layer().with_target(false);

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    let static_files = ServeDir::new("../frontend/dist/");

    let app = Router::new()
        .route("/items/:id", get(get_item))
        .route("/items", get(get_items))
        .route("/items", post(post_item))
        .route("/items/:id/check_out", post(check_out))
        .route("/items/:id/check_in", post(check_in))
        .route("/items/:id/tags/:tag", put(put_tag))
        .route("/items/:id/tags/:tag", delete(delete_tag))
        .nest(
            "/static/",
            get_service(static_files).handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        )
        .layer(Extension(shared_state))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
