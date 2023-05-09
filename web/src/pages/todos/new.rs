use std::ops::Deref;

use serde::Serialize;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{atoms::text_input::TextInput, constants::API_URL, Route};

#[derive(Serialize)]
pub struct PostToDo {
    description: String,
}

#[function_component(ToDoNew)]
pub fn new() -> Html {
    let text = use_state(|| String::new());
    let cloned_text = text.clone();
    let text_input_value = text.deref().clone();
    let handle_onchange = Callback::from(move |value: String| {
        cloned_text.set(value);
    });
    let navigator = use_navigator().unwrap();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let body = PostToDo {
            description: text.deref().clone(),
        };
        let navigator = navigator.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let client = reqwest_wasm::Client::new();
            client.post(API_URL).json(&body).send().await.unwrap();
            navigator.push(&Route::ToDoList);
        });
    });

    html! {
        <form {onsubmit}>
            <TextInput handle_onchange={handle_onchange} value={text_input_value}/>
            <button>{"send"}</button>
        </form>
    }
}
