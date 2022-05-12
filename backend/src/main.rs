use db::Scannable;
use model::Item;

use crate::db::Saveable;

mod db;
mod model;

fn main() {
    let mut conn = rusqlite::Connection::open_in_memory().unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS items (
        id STRING NOT NULL PRIMARY KEY,
        description STRING,
        is_container BOOLEAN NOT NULL,
        checked_out DATETIME,
        destroyed DATETIME,
        parent_container STRING,
        FOREIGN KEY(parent_container) REFERENCES items(id)
    )",
        [],
    )
    .unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tag_associations (
        item_id STRING,
        key STRING,
        value STRING,
        FOREIGN KEY(item_id) REFERENCES items(id),
        UNIQUE(item_id, key)
    )",
        [],
    )
    .unwrap();

    let mut i = Item::new("foo".into(), false, None);
    i.set_tag("tag".into(), "value".into());
    i.save(&mut conn).unwrap();

    eprintln!("{:#?}", Item::scan(&mut conn));
}
