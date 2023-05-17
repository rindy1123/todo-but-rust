use rocket::{http::Status, serde::json::Json, State};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    model::todo::ToDo,
    view::todos::{MultipleToDos, SingleToDo},
};

#[derive(Deserialize)]
pub struct PostToDo {
    description: String,
}

#[derive(Deserialize)]
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
