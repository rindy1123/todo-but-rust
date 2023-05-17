use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

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

type DB = Surreal<Client>;

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

#[cfg(test)]
mod tests {
    use rocket::{
        figment::{
            providers::{Format, Toml},
            Figment,
        },
        futures::future::join_all,
    };
    use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};
    use uuid::Uuid;

    use super::*;

    #[derive(Deserialize)]
    struct DbConfig {
        namespace: String,
        username: String,
        password: String,
        host: String,
        port: String,
    }

    async fn setup_db() -> DB {
        let figment = Figment::new().merge(Toml::file("App.toml").nested());
        let db_conf: DbConfig = figment.select("test_database").extract().unwrap();
        let db = Surreal::new::<Ws>(format!("{}:{}", db_conf.host, db_conf.port))
            .await
            .unwrap();
        db.signin(Root {
            username: &db_conf.username,
            password: &db_conf.password,
        })
        .await
        .unwrap();
        let database = Uuid::new_v4().to_string();
        db.use_ns(db_conf.namespace).use_db(database).await.unwrap();
        db
    }

    async fn mock_todo(db: &DB, description: String) -> ToDo {
        ToDo::create(&db, description).await.unwrap()
    }

    async fn mock_many_todos(db: &DB, number: usize) {
        let mut futures = Vec::new();
        for _ in 0..number {
            futures.push(ToDo::create(&db, "test".to_string()));
        }
        join_all(futures).await;
    }

    #[tokio::test]
    async fn test_create() {
        let db = setup_db().await;
        let todo = ToDo::create(&db, "test".to_string()).await.unwrap();
        assert!(!todo.done);
        assert_eq!(todo.description, "test".to_string());
    }

    #[tokio::test]
    async fn test_get() {
        let db = setup_db().await;
        let ToDo {
            id,
            done: _,
            description: _,
        } = mock_todo(&db, "test".to_string()).await;
        let todo = ToDo::get(&db, id).await.unwrap();
        assert!(todo.is_some());
        assert_eq!(todo.unwrap().description, "test".to_string());
    }

    #[tokio::test]
    async fn test_list() {
        let db = setup_db().await;
        mock_many_todos(&db, 3).await;
        let todos = ToDo::list(&db).await.unwrap();
        assert_eq!(todos.len(), 3);
    }

    #[tokio::test]
    async fn test_update() {
        let db = setup_db().await;
        let ToDo {
            id,
            done: _,
            description: _,
        } = mock_todo(&db, "test".to_string()).await;

        let todo = ToDo::update(&db, id, true, "Done".to_string())
            .await
            .unwrap();
        assert!(todo.done);
        assert_eq!(todo.description, "Done".to_string());
    }

    #[tokio::test]
    async fn test_delete() {
        let db = setup_db().await;
        let ToDo {
            id,
            done: _,
            description: _,
        } = mock_todo(&db, "test".to_string()).await;

        ToDo::delete(&db, id.clone()).await.unwrap();
        let todo = ToDo::get(&db, id).await.unwrap();
        assert!(todo.is_none());
    }
}
