use std::sync::Arc;

use api::*;
use axum::routing::delete;
use axum::routing::get_service;
use axum::routing::post;
use axum::{routing::get, Extension, Router};
use hyper::StatusCode;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

mod api;
mod item;

pub struct State {
    db: RwLock<tokio_postgres::Client>,
}

#[tokio::main]
async fn main() {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres", tokio_postgres::NoTls)
            .await
            .unwrap();

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS items (id UUID PRIMARY KEY);
        CREATE TABLE IF NOT EXISTS tags (
            item_id UUID,
            tag_name TEXT,
            tag_value TEXT,
            PRIMARY KEY (item_id, tag_name),
            FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE
        );
    ",
        )
        .await
        .unwrap();

    let shared_state = Arc::new(State {
        db: RwLock::new(client),
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
        .route("/items/:id", delete(delete_item))
        .route("/items", get(get_items))
        .route("/items", post(post_item))
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
