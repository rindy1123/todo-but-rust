use std::ops::Deref;

use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, HtmlInputElement};
use yew::prelude::*;

const URL: &'static str = "http://localhost:58000/todos";

#[derive(Properties, PartialEq)]
struct TextProps {
    pub handle_onchange: Callback<String>,
}

#[derive(Serialize)]
pub struct PostToDo {
    description: String,
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
#[function_component(ToDoList)]
fn todo_list() -> Html {
    let todo_list_response = use_state(|| ToDoListResponse::default());
    let cloned_todo_list_response = todo_list_response.clone();
    use_effect_with_deps(
        |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let res = reqwest_wasm::get(URL).await.unwrap().text().await.unwrap();
                let todos: ToDoListResponse = serde_json::from_str(&res).unwrap();
                cloned_todo_list_response.set(todos);
            });
        },
        (),
    );
    html! {
        <ul>
            {
                todo_list_response.deref().todos.clone().into_iter().map(|todo| {
                    html!{ <li>{format!("id: {} | {} | {}", todo.id, todo.description, todo.done)}</li> }
                }).collect::<Html>()
            }
        </ul>
    }
}

#[function_component]
fn App() -> Html {
    use_effect(|| {
        wasm_bindgen_futures::spawn_local(async move {
            let res = reqwest_wasm::get(URL).await.unwrap().text().await.unwrap();
            console::log_1(&JsValue::from(res));
        });
    });

    let text = use_state(|| String::new());
    let cloned_text = text.clone();
    let handle_onchange = Callback::from(move |value: String| {
        cloned_text.set(value);
    });

    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let body = PostToDo {
            description: text.deref().clone(),
        };
        wasm_bindgen_futures::spawn_local(async move {
            let client = reqwest_wasm::Client::new();
            let res = client.post(URL).json(&body).send().await.unwrap();
            let res_body = res.text().await.unwrap();
            console::log_1(&JsValue::from(res_body));
        });
    });
    html! {
        <>
            <ToDoList />
            <form {onsubmit}>
                <TextInput handle_onchange={handle_onchange}/>
                <button>{"send"}</button>
            </form>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
