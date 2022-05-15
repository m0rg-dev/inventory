use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Item {
    pub id: Uuid,
    pub description: String,
    pub is_container: bool,
    pub checked_out: Option<NaiveDateTime>,
    pub destroyed: Option<NaiveDateTime>,
    pub parent_container: Option<Uuid>,

    tags: HashMap<String, String>,
}

impl Item {
    pub fn new(description: String, is_container: bool, parent_container: Option<Uuid>) -> Self {
        Item {
            id: Uuid::new_v4(),
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

#[derive(Default)]
pub struct Database(Mutex<DatabaseImpl>);

impl Deref for Database {
    type Target = Mutex<DatabaseImpl>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default)]
pub struct DatabaseImpl {
    contents: HashMap<Uuid, Item>,
}

impl DatabaseImpl {
    const DB_PATH: &'static str = "inventory.json";

    // we can assume that anything in here will not be called concurrently with
    // anything else - all consumers of this API will be using it through a
    // Mutex.

    // TODO this interface sucks
    pub async fn load(&mut self) -> Result<(), std::io::Error> {
        let meta = tokio::fs::metadata(Self::DB_PATH).await;
        if meta.is_err() {
            // save to create the file. if the I/O error was for something other
            // than the file not existing, we'll find out here.
            Self {
                ..Default::default()
            }
            .save()
            .await?;
        }

        let f = tokio::fs::File::open(Self::DB_PATH).await?;
        //let reader = tokio::io::BufReader::new(f);

        // panicking if the DB is invalid JSON seems reasonable here - that's a
        // "come sort it out yourself" sort of situation.
        self.contents = serde_json::from_reader(f.into_std().await).unwrap();

        Ok(())
    }

    pub async fn save(&self) -> Result<(), std::io::Error> {
        let path = format!("{}.tmp-{}", Self::DB_PATH, Uuid::new_v4());

        let tmpfile = tokio::fs::File::create(&path).await?;

        serde_json::to_writer_pretty(tmpfile.into_std().await, &self.contents).unwrap();

        tokio::fs::rename(path, Self::DB_PATH).await
    }
}

impl Deref for DatabaseImpl {
    type Target = HashMap<Uuid, Item>;

    fn deref(&self) -> &Self::Target {
        &self.contents
    }
}

impl DerefMut for DatabaseImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.contents
    }
}
