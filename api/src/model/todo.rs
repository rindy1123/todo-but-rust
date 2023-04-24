use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::DB;

const TABLE_NAME: &'static str = "todo";

#[derive(Serialize, Deserialize, Debug)]
pub struct ToDo {
    pub id: Option<Thing>,
    pub done: bool,
    pub description: String,
}

impl ToDo {
    pub async fn create(db: &DB, description: String) -> Result<ToDo, surrealdb::Error> {
        db.create(TABLE_NAME)
            .content(ToDo {
                id: None,
                done: false,
                description,
            })
            .await
    }

    pub async fn get(db: &DB, id: String) -> Result<Option<ToDo>, surrealdb::Error> {
        db.select((TABLE_NAME, id)).await
    }

    pub async fn list(db: &DB) -> Result<Vec<ToDo>, surrealdb::Error> {
        db.select(TABLE_NAME).await
    }

    pub async fn update(
        db: &DB,
        id: String,
        done: bool,
        description: String,
    ) -> Result<ToDo, surrealdb::Error> {
        db.update((TABLE_NAME, id))
            .merge(ToDo {
                id: None,
                done,
                description,
            })
            .await
    }

    pub async fn delete(db: &DB, id: String) -> Result<Option<ToDo>, surrealdb::Error> {
        db.delete((TABLE_NAME, id)).await
    }
}
