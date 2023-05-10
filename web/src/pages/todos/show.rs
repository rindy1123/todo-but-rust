use std::ops::Deref;

use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{api_client::ApiClient, atoms::text_input::TextInput, structs::ToDo, Route};

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
        let body = ToDo {
            id: cloned_todo.id.clone(),
            done: cloned_todo.done,
            description: cloned_todo.description.clone(),
        };
        let navigator = navigator.clone();
        wasm_bindgen_futures::spawn_local(async move {
            ApiClient::put(body).await;
            navigator.push(&Route::ToDoList);
        });
    });

    let cloned_todo = todo.clone();
    let id = props.id.clone();
    use_effect_with_deps(
        |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let todo: ToDo = ApiClient::get(id).await;
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
