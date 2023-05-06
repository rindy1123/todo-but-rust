use std::ops::Deref;

use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{constants::API_URL, Route};

#[derive(Deserialize, Default, Clone)]
struct ToDoListResponse {
    todos: Vec<ToDo>,
}
#[derive(Deserialize, Default, Clone)]
struct ToDo {
    id: String,
    done: bool,
    description: String,
}

fn to_list_element(todo: ToDo) -> Html {
    let cloned_todo = todo.clone();
    let onclick = Callback::from(move |_| {
        let id = cloned_todo.id.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let client = reqwest_wasm::Client::new();
            client
                .delete(format!("{}/{}", API_URL, id))
                .send()
                .await
                .unwrap();
        });
    });
    html! {
        <li>
            {format!("id: {} | {} | {}", todo.id, todo.description, todo.done)}
            <button {onclick}>{"delete"}</button>
        </li>
    }
}

#[function_component(List)]
fn list() -> Html {
    let todo_list_response = use_state(|| ToDoListResponse::default());
    let cloned_todo_list_response = todo_list_response.clone();
    use_effect_with_deps(
        |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let res = reqwest_wasm::get(API_URL)
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                let todos: ToDoListResponse = serde_json::from_str(&res).unwrap();
                cloned_todo_list_response.set(todos);
            });
        },
        (),
    );
    let todos = todo_list_response.deref().todos.clone().into_iter();

    html! {
        <ul>
            {
                todos.map(to_list_element).collect::<Html>()
            }
        </ul>
    }
}

#[function_component(ToDoList)]
pub fn todo_list() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::ToDoNew));

    html! {
        <>
            <List />
            <button {onclick}>{"create"}</button>
        </>
    }
}
