use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::model::todo::ToDo;

#[derive(Serialize, Deserialize)]
pub struct SingleToDo {
    pub id: String,
    pub done: bool,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct MultipleToDos {
    pub todos: Vec<SingleToDo>,
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

#[cfg(test)]
mod tests {
    use super::*;

    mod single_todo {
        use super::*;

        #[test]
        fn test_generate_response() {
            let todo = ToDo {
                id: "id".to_string(),
                done: false,
                description: "description".to_string(),
            };
            let json = SingleToDo::generate_response(todo);
            assert_eq!(json.id, "id".to_string());
            assert!(!json.done);
            assert_eq!(json.description, "description".to_string());
        }
    }

    mod multiple_todos {
        use super::*;

        #[test]
        fn test_generate_response() {
            let todo1 = ToDo {
                id: "id1".to_string(),
                done: false,
                description: "description1".to_string(),
            };
            let todos = vec![todo1];
            let json = MultipleToDos::generate_response(todos);
            let todo = json.todos.get(0).unwrap();
            assert_eq!(todo.id, "id1".to_string());
            assert!(!todo.done);
            assert_eq!(todo.description, "description1".to_string());
        }
    }
}
