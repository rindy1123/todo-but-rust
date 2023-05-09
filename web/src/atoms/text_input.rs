use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextProps {
    pub handle_onchange: Callback<String>,
    pub value: String,
}

#[function_component(TextInput)]
pub fn text_input(props: &TextProps) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |e: Event| {
        let target = e.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();
        handle_onchange.emit(value);
    });
    let value = props.value.clone();
    html! {
        <input type="text" {onchange} {value}/>
    }
}
