use wasm_bindgen::JsValue;
use web_sys::console;
use yew::prelude::*;

const URL: &'static str = "http://localhost:58000/todos";

#[function_component]
fn App() -> Html {
    wasm_bindgen_futures::spawn_local(async move {
        let res = reqwest_wasm::get(URL).await.unwrap().text().await.unwrap();
        console::log_1(&JsValue::from(res));
    });
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };
    html! {
        <div>
            <button {onclick}>{"+1"}</button>
            <span>{ *counter }</span>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
