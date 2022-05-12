use std::{collections::HashMap, vec};

use chrono::NaiveDateTime;
use rusqlite::named_params;
use uuid::Uuid;

use crate::db::{FromRow, LoadableBy, Obj, Saveable};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Item {
    pub id: String,
    pub description: String,
    pub is_container: bool,
    pub checked_out: Option<NaiveDateTime>,
    pub destroyed: Option<NaiveDateTime>,
    pub parent_container: Option<String>,

    tags: HashMap<String, String>,
}

impl Item {
    pub fn new(description: String, is_container: bool, parent_container: Option<String>) -> Self {
        Item {
            id: Uuid::new_v4().to_string(),
            description,
            is_container,
            checked_out: None,
            destroyed: None,
            parent_container,
            tags: HashMap::new(),
        }
    }

    pub fn set_tag(&mut self, key: String, value: String) {
        self.tags.insert(key, value);
    }
}

impl Obj for Item {
    type Id = String;

    fn table_name() -> String {
        "items".into()
    }

    fn where_clause() -> String {
        "id = :id".into()
    }

    fn where_params(id: &Self::Id) -> Vec<(&str, &dyn rusqlite::ToSql)> {
        vec![("id", id)]
    }

    fn key_columns(&self) -> Vec<(&str, &dyn rusqlite::ToSql)> {
        named_params! {
            "id": self.id
        }
        .to_vec()
    }

    fn data_columns(&self) -> Vec<(&str, &dyn rusqlite::ToSql)> {
        named_params! {
            "description": self.description,
            "is_container": self.is_container,
            "checked_out": self.checked_out,
            "destroyed": self.destroyed,
            "parent_container": self.parent_container,
        }
        .to_vec()
    }

    fn post_save_hook(&self, conn: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        self.tags
            .clone()
            .into_iter()
            .map(|(k, v)| Tag {
                item_id: self.id.clone(),
                key: k,
                value: v,
            })
            .try_for_each(|t| t.__save_no_savepoint(conn))?;

        Ok(())
    }

    fn post_load_hook(&mut self, conn: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        let tags = Tag::load_by(&TagLoadableBy::ItemId(self.id.clone()), conn)?;

        tags.into_iter().for_each(|t| self.set_tag(t.key, t.value));

        Ok(())
    }
}

impl FromRow for Item {
    fn from_row<'row, 'stmt: 'row>(
        row: &'row rusqlite::Row<'stmt>,
    ) -> Result<Self, rusqlite::Error> {
        Ok(Item {
            id: row.get(0)?,
            description: row.get(1)?,
            is_container: row.get(2)?,
            checked_out: row.get(3)?,
            destroyed: row.get(4)?,
            parent_container: row.get(5)?,
            tags: HashMap::new(),
        })
    }
}

#[allow(dead_code)]
pub enum ItemLoadableBy {
    ParentContainer(String),
}

impl LoadableBy<ItemLoadableBy> for Item {
    fn select_by(by: &ItemLoadableBy) -> (String, Vec<(&str, &dyn rusqlite::ToSql)>) {
        match by {
            ItemLoadableBy::ParentContainer(id) => (
                "SELECT * FROM items WHERE parent_container = :parent_container".into(),
                vec![("parent_container", id)],
            ),
        }
    }
}

#[allow(dead_code)]
pub(crate) struct Tag {
    pub item_id: String,
    pub key: String,
    pub value: String,
}

#[allow(dead_code)]
pub enum TagLoadableBy {
    ItemId(String),
    Key(String),
}

impl Obj for Tag {
    type Id = (String, String);

    fn table_name() -> String {
        "tag_associations".into()
    }

    fn where_clause() -> String {
        "WHERE item_id = :item_id AND key = :key".into()
    }

    fn where_params((item_id, key): &Self::Id) -> Vec<(&str, &dyn rusqlite::ToSql)> {
        vec![(":item_id", item_id), (":key", key)]
    }

    fn key_columns(&self) -> Vec<(&str, &dyn rusqlite::ToSql)> {
        vec![("item_id", &self.item_id), ("key", &self.key)]
    }

    fn data_columns(&self) -> Vec<(&str, &dyn rusqlite::ToSql)> {
        vec![("value", &self.value)]
    }
}

impl LoadableBy<TagLoadableBy> for Tag {
    fn select_by(by: &TagLoadableBy) -> (String, Vec<(&str, &dyn rusqlite::ToSql)>) {
        match by {
            TagLoadableBy::ItemId(id) => (
                "SELECT * FROM tag_associations WHERE item_id = :item_id".into(),
                vec![(":item_id", id)],
            ),
            TagLoadableBy::Key(key) => (
                "SELECT * FROM tag_associations WHERE key = :key".into(),
                vec![(":key", key)],
            ),
        }
    }
}

impl FromRow for Tag {
    fn from_row<'row, 'stmt: 'row>(
        row: &'row rusqlite::Row<'stmt>,
    ) -> Result<Self, rusqlite::Error> {
        Ok(Tag {
            item_id: row.get(0)?,
            key: row.get(1)?,
            value: row.get(2)?,
        })
    }
}
