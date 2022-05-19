use std::{fmt::Display, process::Stdio, sync::Arc};

use axum::{
    extract::{Path, RawBody},
    http::StatusCode,
    Extension, Json,
};
use axum_macros::debug_handler;
use tokio::{io::AsyncWriteExt, process::Command};
use tokio_postgres::types::Type;
use tracing::{event, Level};
use uuid::Uuid;

use crate::{item::Item, State};

#[debug_handler]
pub async fn get_item(
    Path(id): Path<Uuid>,
    Extension(state): Extension<Arc<State>>,
) -> Result<Json<Item>, (StatusCode, String)> {
    let db = handle_error(state.db.get().await)?;
    let stmt = handle_error(db.prepare_cached("SELECT 1 FROM items WHERE id=$1").await)?;
    let i = handle_error(db.query_opt(&stmt, &[&id]).await)?;

    if i.is_none() {
        return Err((StatusCode::NOT_FOUND, "Not Found\n".into()));
    }

    let stmt = handle_error(
        db.prepare_cached("SELECT * FROM tags WHERE item_id=$1")
            .await,
    )?;

    let rows = handle_error(db.query(&stmt, &[&id]).await)?;

    Ok(Json(Item::from_tag_rows(id, rows)))
}

#[debug_handler]
pub async fn get_items(
    Extension(state): Extension<Arc<State>>,
) -> Result<Json<Vec<Uuid>>, (StatusCode, String)> {
    let db = handle_error(state.db.get().await)?;
    let stmt = handle_error(db.prepare_cached("SELECT id FROM items").await)?;
    let rows = handle_error(db.query(&stmt, &[]).await)?;

    Ok(Json(rows.iter().map(|r| r.get(0)).collect()))
}

#[debug_handler]
pub async fn post_item(
    Json(item): Json<Item>,
    Extension(state): Extension<Arc<State>>,
) -> Result<Json<Item>, (StatusCode, String)> {
    let mut db = handle_error(state.db.get().await)?;
    let tx = handle_error(db.transaction().await)?;

    let stmt = handle_error(
        tx.prepare_cached("INSERT INTO items (id) VALUES ($1) ON CONFLICT DO NOTHING")
            .await,
    )?;
    handle_error(tx.execute(&stmt, &[&item.id]).await)?;

    for (k, v) in &item.tags {
        let stmt = handle_error(
            tx.prepare_cached(
                "INSERT INTO tags (item_id, tag_name, tag_value)
                          VALUES ($1, $2, $3)
                          ON CONFLICT (item_id, tag_name) DO UPDATE SET tag_value = EXCLUDED.tag_value",
            )
            .await,
        )?;
        handle_error(tx.execute(&stmt, &[&item.id, k, v]).await)?;
    }

    let stmt = handle_error(
        tx.prepare_typed_cached(
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

#[debug_handler]
pub async fn delete_item(
    Path(id): Path<Uuid>,
    Extension(state): Extension<Arc<State>>,
) -> Result<(), (StatusCode, String)> {
    handle_error(
        state
            .db
            .get()
            .await
            .unwrap()
            .execute("DELETE FROM items WHERE id=$1", &[&id])
            .await,
    )?;

    Ok(())
}

pub async fn print_label(RawBody(b): RawBody) -> Result<(), (StatusCode, String)> {
    let mut cmd = handle_error(
        Command::new("sh")
            .arg("-c")
            .arg("brother_ql -b linux_kernel -p file:///dev/usb/lp0 -m QL-500 print -l 29 -")
            .stdin(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn(),
    )?;

    handle_error(
        cmd.stdin
            .take()
            .unwrap()
            .write(&handle_error(hyper::body::to_bytes(b).await)?)
            .await,
    )?;

    handle_error(cmd.wait().await)?;

    Ok(())
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
