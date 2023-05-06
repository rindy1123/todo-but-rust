use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::DB;

const TABLE_NAME: &'static str = "todo";

/// Raw data returned by DB
#[derive(Serialize, Deserialize)]
struct ToDoResource {
    id: Option<Thing>,
    done: bool,
    description: String,
}

pub struct ToDo {
    pub id: String,
    pub done: bool,
    pub description: String,
}

impl ToDo {
    pub async fn create(db: &DB, description: String) -> Result<ToDo, surrealdb::Error> {
        let todo = db
            .create(TABLE_NAME)
            .content(ToDoResource {
                id: None,
                done: false,
                description,
            })
            .await?;
        Ok(Self::raw_data_to_todo(todo))
    }

    pub async fn get(db: &DB, id: String) -> Result<Option<ToDo>, surrealdb::Error> {
        let todo: Option<ToDoResource> = db.select((TABLE_NAME, id)).await?;
        match todo {
            Some(todo) => Ok(Some(Self::raw_data_to_todo(todo))),
            None => Ok(None),
        }
    }

    pub async fn list(db: &DB) -> Result<Vec<ToDo>, surrealdb::Error> {
        let todos = db.select(TABLE_NAME).await?.into_iter();
        Ok(todos.map(Self::raw_data_to_todo).collect())
    }

    pub async fn update(
        db: &DB,
        id: String,
        done: bool,
        description: String,
    ) -> Result<ToDo, surrealdb::Error> {
        let todo = db
            .update((TABLE_NAME, id))
            .merge(ToDoResource {
                id: None,
                done,
                description,
            })
            .await?;
        Ok(Self::raw_data_to_todo(todo))
    }

    pub async fn delete(db: &DB, id: String) -> Result<Option<()>, surrealdb::Error> {
        let todo: Option<ToDoResource> = db.delete((TABLE_NAME, id)).await?;
        match todo {
            Some(_) => Ok(Some(())),
            None => Ok(None),
        }
    }

    fn raw_data_to_todo(todo: ToDoResource) -> ToDo {
        let id = todo.id.clone().unwrap().id.to_raw();
        ToDo {
            id,
            done: todo.done,
            description: todo.description,
        }
    }
}
