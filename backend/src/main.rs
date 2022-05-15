use std::sync::Arc;

use api::get_item;
use api::post_item;
use api::put_tag;
use axum::routing::post;
use axum::routing::put;
use axum::{routing::get, Extension, Router};
use model::Database;
use tower::ServiceBuilder;
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

    let app = Router::new()
        .route("/items/:id", get(get_item))
        .route("/items", post(post_item))
        .route("/items/:id/tags/:tag", put(put_tag))
        .layer(Extension(shared_state))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
