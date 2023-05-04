use serde::Serialize;
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
            description: (*text).clone(),
        };
        wasm_bindgen_futures::spawn_local(async move {
            let client = reqwest_wasm::Client::new();
            let res = client.post(URL).json(&body).send().await.unwrap();
            let res_body = res.text().await.unwrap();
            console::log_1(&JsValue::from(res_body));
        });
    });
    html! {
        <form {onsubmit}>
            <TextInput handle_onchange={handle_onchange}/>
            <button>{"send"}</button>
        </form>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
