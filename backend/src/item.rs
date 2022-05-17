use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub tags: HashMap<String, String>,
}

impl Item {
    // expects a list of (id, key, value) rows
    pub fn from_tag_rows(id: Uuid, rows: Vec<Row>) -> Item {
        let mut tags = HashMap::new();

        rows.into_iter().for_each(|r| {
            tags.insert(r.get(1), r.get(2));
        });

        Item { id, tags }
    }
}
