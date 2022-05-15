use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, RawBody},
    http::StatusCode,
    Extension, Json,
};
use axum_macros::debug_handler;
use serde::Deserialize;
use tracing::{event, Level};
use uuid::Uuid;

use crate::{model::Item, State};

#[debug_handler]
pub async fn get_item(
    Path(id): Path<Uuid>,
    Extension(state): Extension<Arc<State>>,
) -> Result<Json<Item>, (StatusCode, String)> {
    let mut db = state.db.lock().await;
    handle_io_error(db.load().await)?;

    let item = db.get(&id);

    match item {
        Some(item) => Ok(Json(item.clone())),
        None => Err((StatusCode::NOT_FOUND, "Not Found\n".into())),
    }
}

#[derive(Deserialize)]
pub struct PostItem {
    description: String,
    #[serde(default)]
    is_container: bool,
    #[serde(default)]
    tags: HashMap<String, String>,
    parent_container: Option<Uuid>,
}

#[debug_handler]
pub async fn post_item(
    Json(args): Json<PostItem>,
    Extension(state): Extension<Arc<State>>,
) -> Result<Json<Item>, (StatusCode, String)> {
    let mut db = state.db.lock().await;
    handle_io_error(db.load().await)?;

    let mut i = Item::new(args.description, args.is_container, args.parent_container);

    for (k, v) in args.tags {
        i.set_tag(k, v);
    }

    db.insert(i.id, i.clone());

    handle_io_error(db.save().await)?;

    Ok(Json(i))
}

#[debug_handler]
pub async fn put_tag(
    Path((id, tag)): Path<(Uuid, String)>,
    RawBody(body): RawBody,
    Extension(state): Extension<Arc<State>>,
) -> Result<Json<Item>, (StatusCode, String)> {
    let body = hyper::body::to_bytes(body)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut db = state.db.lock().await;
    handle_io_error(db.load().await)?;

    let item = db.get(&id).cloned();
    if let Some(mut item) = item {
        item.set_tag(tag, String::from_utf8_lossy(&body).to_string());
        db.insert(item.id, item.clone());

        handle_io_error(db.save().await)?;

        Ok(Json(item))
    } else {
        Err((StatusCode::NOT_FOUND, "Not Found\n".into()))
    }
}

fn handle_io_error<T>(r: Result<T, std::io::Error>) -> Result<T, (StatusCode, String)> {
    r.map_err(|e| {
        event!(Level::ERROR, "Internal server error: {e}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal server error: {e}\n"),
        )
    })
}
