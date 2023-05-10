use std::ops::Deref;

use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    api_client::{ApiClient, ToDoListResponse},
    structs::ToDo,
    Route,
};

#[derive(Properties, PartialEq)]
struct DeleteButtonProps {
    id: String,
}

#[function_component(DeleteButton)]
fn delete_button(props: &DeleteButtonProps) -> Html {
    let id = props.id.clone();
    let onclick = Callback::from(move |_| {
        let id = id.clone();
        wasm_bindgen_futures::spawn_local(async move {
            ApiClient::delete(id).await;
        });
    });
    html! {
        <button {onclick}>{"delete"}</button>
    }
}

#[function_component(ToDoItem)]
fn todo_item(todo: &ToDo) -> Html {
    let cloned_todo = todo.clone();
    let navigator = use_navigator().unwrap();
    let onclick_checkbox = Callback::from(move |e: MouseEvent| {
        let target = e.target().unwrap();
        let done = target.unchecked_into::<HtmlInputElement>().checked();
        let id = cloned_todo.id.clone();
        let description = cloned_todo.description.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let params = ToDo {
                id,
                done,
                description,
            };
            ApiClient::put(params).await;
        });
    });

    let cloned_todo = todo.clone();
    let onclick_label = Callback::from(move |_| {
        let id = cloned_todo.id.clone();
        navigator.push(&Route::ToDoShow { id })
    });

    html! {
        <div>
            <input
                type="checkbox"
                id={todo.id.clone()}
                name={todo.id.clone()}
                checked={todo.done}
                onclick={onclick_checkbox}
            />
            <label for={todo.id.clone()} onclick={onclick_label}>{todo.description.clone()}</label>
            <DeleteButton id={todo.id.clone()} />
        </div>
    }
}

#[function_component(List)]
fn list() -> Html {
    let todo_list_response = use_state(|| ToDoListResponse::default());
    let cloned_todo_list_response = todo_list_response.clone();
    use_effect_with_deps(
        |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let todos = ApiClient::list().await;
                cloned_todo_list_response.set(todos);
            });
        },
        (),
    );
    let todos = todo_list_response.deref().todos.clone().into_iter();

    html! {
        {
            todos.map(|todo| {
                html!{ <ToDoItem id={todo.id} description={todo.description} done={todo.done} /> }
            }).collect::<Html>()
        }
    }
}

#[function_component(ToDoList)]
pub fn todo_list() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::ToDoNew));

    html! {
        <>
            <h1>{"TODO List"}</h1>
            <List />
            <button {onclick}>{"create"}</button>
        </>
    }
}
