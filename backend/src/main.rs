use std::sync::Arc;

// use api::get_item;
// use api::post_item;
// use api::put_tag;
use axum::routing::post;
use axum::routing::put;
use axum::{routing::get, Extension, Router};
use model::Item;
use sqlx::SqlitePool;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

use crate::db::{Saveable, Scannable};

// mod api;
mod db;
mod model;

pub struct State {
    db_conn: SqlitePool,
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    let conn = rusqlite::Connection::open_in_memory().unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS items (
        id STRING NOT NULL PRIMARY KEY,
        description STRING,
        is_container BOOLEAN NOT NULL,
        checked_out DATETIME,
        destroyed DATETIME,
        parent_container STRING,
        FOREIGN KEY(parent_container) REFERENCES items(id)
    )",
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tag_associations (
        id INTEGERR PRIMARY KEY,
        item_id STRING NOT NULL,
        key STRING NOT NULL,
        value STRING NOT NULL,
        FOREIGN KEY(item_id) REFERENCES items(id),
        UNIQUE(item_id, key)
    )",
    )
    .execute(&pool)
    .await
    .unwrap();

    // let shared_state = Arc::new(State { db_conn: pool });

    // let filter_layer = EnvFilter::try_from_default_env()
    //     .or_else(|_| EnvFilter::try_new("info"))
    //     .unwrap();

    // let fmt_layer = tracing_subscriber::fmt::layer().with_target(false);

    // tracing_subscriber::registry()
    //     .with(filter_layer)
    //     .with(fmt_layer)
    //     .init();

    // let app = Router::new()
    //     .route("/items/:id", get(get_item))
    //     .route("/items", post(post_item))
    //     .route("/items/:id/tags/:tag", put(put_tag))
    //     .layer(Extension(shared_state))
    //     .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    // axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}
