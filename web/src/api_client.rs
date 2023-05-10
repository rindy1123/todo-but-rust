use serde::{Deserialize, Serialize};

use crate::structs::ToDo;

#[derive(Serialize)]
pub struct PostToDo {
    description: String,
}

pub const API_URL: &'static str = "http://localhost:58000/todos";

impl PostToDo {
    pub fn new(description: String) -> PostToDo {
        PostToDo { description }
    }
}

#[derive(Deserialize, Default, Clone)]
pub struct ToDoListResponse {
    pub todos: Vec<ToDo>,
}

pub struct ApiClient;

impl ApiClient {
    pub async fn post(body: PostToDo) {
        let client = reqwest_wasm::Client::new();
        client.post(API_URL).json(&body).send().await.unwrap();
    }

    pub async fn put(body: ToDo) {
        let client = reqwest_wasm::Client::new();
        client
            .put(format!("{}/{}", API_URL, body.id))
            .json(&body)
            .send()
            .await
            .unwrap();
    }

    pub async fn get(id: String) -> ToDo {
        let res = reqwest_wasm::get(format!("{}/{}", API_URL, id))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        serde_json::from_str(&res).unwrap()
    }

    pub async fn list() -> ToDoListResponse {
        let res = reqwest_wasm::get(API_URL)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        serde_json::from_str(&res).unwrap()
    }

    pub async fn delete(id: String) {
        let client = reqwest_wasm::Client::new();
        client
            .delete(format!("{}/{}", API_URL, id))
            .send()
            .await
            .unwrap();
    }
}
