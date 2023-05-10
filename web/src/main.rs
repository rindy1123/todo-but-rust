use pages::todos::{list::ToDoList, new::ToDoNew, show::ToDoShow};
use yew::prelude::*;
use yew_router::prelude::*;

mod api_client;
mod atoms;
mod pages;
mod structs;

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/todos")]
    ToDoList,
    #[at("/todos/:id")]
    ToDoShow { id: String },
    #[at("/todos/new")]
    ToDoNew,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Redirect<Route> to={Route::ToDoList} /> },
        Route::ToDoList => html! { <ToDoList /> },
        Route::ToDoShow { id } => html! { <ToDoShow id={id} /> },
        Route::ToDoNew => html! { <ToDoNew /> },
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
