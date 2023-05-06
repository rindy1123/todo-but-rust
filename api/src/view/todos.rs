use rocket::serde::json::Json;
use serde::Serialize;

use crate::model::todo::ToDo;

#[derive(Serialize)]
pub struct SingleToDo {
    id: String,
    done: bool,
    description: String,
}

#[derive(Serialize)]
pub struct MultipleToDos {
    todos: Vec<SingleToDo>,
}

impl SingleToDo {
    pub fn generate_response(todo: ToDo) -> Json<SingleToDo> {
        let res = SingleToDo {
            id: todo.id,
            done: todo.done,
            description: todo.description,
        };
        Json(res)
    }
}

impl MultipleToDos {
    pub fn generate_response(todos: Vec<ToDo>) -> Json<MultipleToDos> {
        let res: Vec<SingleToDo> = todos
            .into_iter()
            .map(|todo| SingleToDo {
                id: todo.id,
                done: todo.done,
                description: todo.description,
            })
            .collect();
        Json(MultipleToDos { todos: res })
    }
}
