use rocket::{http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    model::todo::ToDo,
    view::todos::{MultipleToDos, SingleToDo},
};

#[derive(Serialize, Deserialize)]
pub struct PostToDo {
    description: String,
}

#[derive(Serialize, Deserialize)]
pub struct PutToDo {
    done: bool,
    description: String,
}

type DB = State<Surreal<Client>>;

#[post("/todos", data = "<todo>")]
pub async fn create(todo: Json<PostToDo>, db: &DB) -> Result<Json<SingleToDo>, Status> {
    match ToDo::create(db.inner(), todo.description.clone()).await {
        Ok(todo) => {
            let response = SingleToDo::generate_response(todo);
            Ok(response)
        }
        Err(err) => {
            println!("{err}");
            Err(Status::InternalServerError)
        }
    }
}

#[get("/todos/<id>")]
pub async fn show(id: String, db: &DB) -> Result<Json<SingleToDo>, Status> {
    match ToDo::get(db, id).await {
        Ok(todo) => {
            if let Some(todo) = todo {
                let response = SingleToDo::generate_response(todo);
                Ok(response)
            } else {
                println!("ToDo Not Found");
                Err(Status::NotFound)
            }
        }
        Err(err) => {
            println!("{err}");
            Err(Status::InternalServerError)
        }
    }
}

#[get("/todos")]
pub async fn index(db: &DB) -> Result<Json<MultipleToDos>, Status> {
    match ToDo::list(db).await {
        Ok(todos) => {
            let response = MultipleToDos::generate_response(todos);
            Ok(response)
        }
        Err(err) => {
            println!("{err}");
            Err(Status::InternalServerError)
        }
    }
}

#[put("/todos/<id>", data = "<todo>")]
pub async fn update(id: String, todo: Json<PutToDo>, db: &DB) -> Result<Json<SingleToDo>, Status> {
    match ToDo::update(db, id, todo.done, todo.description.clone()).await {
        Ok(todo) => {
            let response = SingleToDo::generate_response(todo);
            Ok(response)
        }
        Err(err) => {
            println!("{err}");
            Err(Status::InternalServerError)
        }
    }
}

#[delete("/todos/<id>")]
pub async fn delete(id: String, db: &DB) -> Status {
    match ToDo::delete(db, id).await {
        Ok(todo) => {
            if let None = todo {
                Status::NotFound
            } else {
                Status::NoContent
            }
        }
        Err(err) => {
            println!("{err}");
            Status::InternalServerError
        }
    }
}

#[cfg(test)]
mod test {
    use rocket::{
        figment::providers::{Format, Toml},
        local::blocking::Client,
        Config,
    };

    use crate::fairing::db::TestDbMiddleware;

    use super::*;

    fn create_client() -> Client {
        let todo_api = routes![index, show, create, update, delete];
        let figment = Config::figment().merge(Toml::file("App.toml").nested());
        let rocket = rocket::custom(figment)
            .mount("/", todo_api)
            .attach(TestDbMiddleware);
        Client::tracked(rocket).unwrap()
    }

    #[test]
    fn test_create() {
        let client = create_client();
        let body = PostToDo {
            description: "test".to_string(),
        };
        let uri = uri!(create);
        let response = client.post(uri).json(&body).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let json: SingleToDo = response.into_json().unwrap();
        assert!(!json.id.is_empty())
    }

    #[test]
    fn test_show() {
        let client = create_client();
        let body = PostToDo {
            description: "test".to_string(),
        };
        let uri = uri!(create);
        let response = client.post(uri).json(&body).dispatch();
        let SingleToDo {
            id,
            done: _,
            description: _,
        } = response.into_json().unwrap();
        let uri = uri!(show(id));
        let response = client.get(uri).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let json: SingleToDo = response.into_json().unwrap();
        assert_eq!(json.description, "test".to_string());
    }

    #[test]
    fn test_index() {
        let client = create_client();
        let uri = uri!(create);
        let body1 = PostToDo {
            description: "test1".to_string(),
        };
        client.post(uri.clone()).json(&body1).dispatch();
        let body2 = PostToDo {
            description: "test2".to_string(),
        };
        client.post(uri).json(&body2).dispatch();

        let uri = uri!(index);
        let response = client.get(uri).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let MultipleToDos { todos } = response.into_json().unwrap();
        assert_eq!(todos.len(), 2);
    }

    #[test]
    fn test_update() {
        let client = create_client();
        let body = PostToDo {
            description: "test".to_string(),
        };
        let uri = uri!(create);
        let response = client.post(uri).json(&body).dispatch();

        let SingleToDo {
            id,
            done: _,
            description: _,
        } = response.into_json().unwrap();
        let uri = uri!(update(id));
        let body = PutToDo {
            done: true,
            description: "test2".to_string(),
        };
        let response = client.put(uri).json(&body).dispatch();

        assert_eq!(response.status(), Status::Ok);
        let json: SingleToDo = response.into_json().unwrap();
        assert_eq!(json.description, "test2".to_string());
        assert!(json.done);
    }

    #[test]
    fn test_delete() {
        let client = create_client();
        let body = PostToDo {
            description: "test".to_string(),
        };
        let uri = uri!(create);
        let response = client.post(uri).json(&body).dispatch();

        let SingleToDo {
            id,
            done: _,
            description: _,
        } = response.into_json().unwrap();
        let uri = uri!(delete(id));
        let response = client.delete(uri).dispatch();

        assert_eq!(response.status(), Status::NoContent);
    }
}
