use std::sync::Arc;

use api::*;
use axum::routing::delete;
use axum::routing::get_service;
use axum::routing::post;
use axum::{routing::get, Extension, Router};
use deadpool_postgres::Config;
use deadpool_postgres::ManagerConfig;
use deadpool_postgres::PoolConfig;
use deadpool_postgres::Runtime;
use hyper::StatusCode;
use tokio_postgres::NoTls;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

mod api;
mod item;

pub struct State {
    db: deadpool_postgres::Pool,
}

#[tokio::main]
async fn main() {
    let mut db_cfg = Config::new();
    db_cfg.host = Some("localhost".into());
    db_cfg.user = Some("postgres".into());
    db_cfg.dbname = Some("postgres".into());
    db_cfg.manager = Some(ManagerConfig {
        recycling_method: deadpool_postgres::RecyclingMethod::Fast,
    });
    db_cfg.pool = Some(PoolConfig {
        max_size: 128,
        timeouts: Default::default(),
    });

    let pool = db_cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    let client = pool.get().await.unwrap();

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

    let shared_state = Arc::new(State { db: pool });

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    let fmt_layer = tracing_subscriber::fmt::layer().with_target(false);

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    let static_files = ServeDir::new("./dist/");

    let app = Router::new()
        .route("/api/items/:id", get(get_item))
        .route("/api/items/:id", delete(delete_item))
        .route("/api/items", get(get_items))
        .route("/api/items", post(post_item))
        .route("/api/label", post(print_label))
        .fallback(
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
