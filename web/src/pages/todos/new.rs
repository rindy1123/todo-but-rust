use std::ops::Deref;

use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    api_client::{ApiClient, PostToDo},
    atoms::text_input::TextInput,
    Route,
};

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
        let body = PostToDo::new(text.deref().clone());
        let navigator = navigator.clone();
        wasm_bindgen_futures::spawn_local(async move {
            ApiClient::post(body).await;
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
