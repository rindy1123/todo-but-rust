use std::ops::Deref;

use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{constants::API_URL, Route};

#[derive(Serialize)]
pub struct PostToDo {
    description: String,
}

#[derive(Properties, PartialEq)]
struct TextProps {
    pub handle_onchange: Callback<String>,
}

#[function_component(TextInput)]
fn text_input(props: &TextProps) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |e: Event| {
        let target = e.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();
        handle_onchange.emit(value);
    });
    html! {
        <input type="text" {onchange} />
    }
}

#[function_component(ToDoNew)]
pub fn new() -> Html {
    let text = use_state(|| String::new());
    let cloned_text = text.clone();
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
            <TextInput handle_onchange={handle_onchange}/>
            <button>{"send"}</button>
        </form>
    }
}
