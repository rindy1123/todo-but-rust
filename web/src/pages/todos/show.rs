use serde::Deserialize;
use yew::prelude::*;

use crate::constants::API_URL;

#[derive(Deserialize, Default, Clone, PartialEq, Properties)]
struct ToDo {
    id: String,
    done: bool,
    description: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

// TODO: update description
#[function_component(ToDoShow)]
pub fn show(props: &Props) -> Html {
    let todo = use_state(|| ToDo::default());
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

    html! {
        <div>{&*todo.description}</div>
    }
}
