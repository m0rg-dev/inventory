use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{BodyStream, Path, Query, RawBody},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use axum_macros::debug_handler;
use serde::Deserialize;
use tracing::{event, Level};
use uuid::Uuid;

use crate::{
    db::{Loadable, Saveable},
    model::Item,
    State,
};

#[debug_handler]
pub async fn get_item(
    Path(id): Path<Uuid>,
    Extension(state): Extension<Arc<State>>,
) -> Result<Json<Item>, (StatusCode, String)> {
    let conn = state.db_conn.lock().await;

    let item = handle_sql_error(Item::load(id.to_string(), &conn))?;

    match item {
        Some(item) => Ok(Json(item)),
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
    let mut conn = state.db_conn.lock().await;

    let mut i = Item::new(
        args.description,
        args.is_container,
        args.parent_container.map(|u| u.to_string()),
    );

    for (k, v) in args.tags {
        i.set_tag(k, v);
    }

    handle_sql_error(i.save(&mut conn))?;

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

    let mut conn = state.db_conn.lock().await;
    let mut tx = handle_sql_error(conn.transaction())?;

    let item = handle_sql_error(Item::load(id.to_string(), &tx))?;
    if let Some(mut item) = item {
        item.set_tag(tag, String::from_utf8_lossy(&body).to_string());
        handle_sql_error(item.save(&mut tx))?;
        return Ok(Json(item));
    } else {
        return Err((StatusCode::NOT_FOUND, "Not Found\n".into()));
    }
}

fn handle_sql_error<T>(r: Result<T, rusqlite::Error>) -> Result<T, (StatusCode, String)> {
    r.map_err(|e| {
        event!(Level::ERROR, "Internal database error: {e}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal database error: {e}\n"),
        )
    })
}
