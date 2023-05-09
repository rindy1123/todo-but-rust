use std::ops::Deref;

use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{atoms::text_input::TextInput, constants::API_URL, Route};

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Properties)]
struct ToDo {
    id: String,
    done: bool,
    description: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(ToDoShow)]
pub fn show(props: &Props) -> Html {
    let todo = use_state(|| ToDo::default());
    let navigator = use_navigator().unwrap();

    let cloned_todo = todo.clone();
    let handle_onchange = Callback::from(move |value: String| {
        let ToDo {
            id,
            done,
            description: _,
        } = cloned_todo.deref();
        cloned_todo.set(ToDo {
            id: id.clone(),
            done: *done,
            description: value,
        });
    });

    let cloned_todo = todo.clone();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let body = cloned_todo.deref().clone();
        let navigator = navigator.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let client = reqwest_wasm::Client::new();
            client
                .put(format!("{}/{}", API_URL, body.id))
                .json(&body)
                .send()
                .await
                .unwrap();
            navigator.push(&Route::ToDoList);
        });
    });

    let cloned_todo = todo.clone();
    let id = props.id.clone();
    use_effect_with_deps(
        |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let res = reqwest_wasm::get(format!("{}/{}", API_URL, id))
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                let todo: ToDo = serde_json::from_str(&res).unwrap();
                cloned_todo.set(todo);
            });
        },
        (),
    );

    let description = todo.description.clone();
    html! {
        <form {onsubmit}>
            <TextInput {handle_onchange} value={description}/>
            <button>{"send"}</button>
        </form>
    }
}
