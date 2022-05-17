use std::{fmt::Display, sync::Arc};

use axum::{extract::Path, http::StatusCode, Extension, Json};
use axum_macros::debug_handler;
use tokio_postgres::types::Type;
use tracing::{event, Level};
use uuid::Uuid;

use crate::{item::Item, State};

#[debug_handler]
pub async fn get_item(
    Path(id): Path<Uuid>,
    Extension(state): Extension<Arc<State>>,
) -> Result<Json<Item>, (StatusCode, String)> {
    let rows = handle_error(
        state
            .db
            .read()
            .await
            .query("SELECT * FROM tags WHERE item_id=$1", &[&id])
            .await,
    )?;

    if !rows.is_empty() {
        Ok(Json(Item::from_tag_rows(id, rows)))
    } else {
        Err((StatusCode::NOT_FOUND, "Not Found\n".into()))
    }
}

#[debug_handler]
pub async fn get_items(
    Extension(state): Extension<Arc<State>>,
) -> Result<Json<Vec<Uuid>>, (StatusCode, String)> {
    let rows = handle_error(
        state
            .db
            .read()
            .await
            .query("SELECT id FROM items", &[])
            .await,
    )?;

    Ok(Json(rows.iter().map(|r| r.get(0)).collect()))
}

#[debug_handler]
pub async fn post_item(
    Json(item): Json<Item>,
    Extension(state): Extension<Arc<State>>,
) -> Result<Json<Item>, (StatusCode, String)> {
    let mut db = state.db.write().await;
    let tx = handle_error(db.transaction().await)?;

    handle_error(
        tx.execute(
            "INSERT INTO items (id) VALUES ($1) ON CONFLICT DO NOTHING",
            &[&item.id],
        )
        .await,
    )?;

    for (k, v) in &item.tags {
        handle_error(
            tx.execute(
                "INSERT INTO tags (item_id, tag_name, tag_value)
                              VALUES ($1, $2, $3)
                              ON CONFLICT (item_id, tag_name) DO UPDATE SET tag_value = EXCLUDED.tag_value",
                &[&item.id, k, v],
            )
            .await,
        )?;
    }

    let stmt = handle_error(
        tx.prepare_typed(
            "DELETE FROM tags WHERE item_id=$1 AND tag_name <> ALL($2)",
            &[Type::UUID, Type::TEXT_ARRAY],
        )
        .await,
    )?;

    handle_error(
        tx.execute(
            &stmt,
            &[&item.id, &item.tags.keys().cloned().collect::<Vec<_>>()],
        )
        .await,
    )?;

    handle_error(tx.commit().await)?;

    Ok(Json(item))
}

fn handle_error<T, E: Display>(r: Result<T, E>) -> Result<T, (StatusCode, String)> {
    r.map_err(|e| {
        event!(Level::ERROR, "Internal server error: {e}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal server error: {e}\n"),
        )
    })
}
