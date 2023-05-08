use pages::todos::{list::ToDoList, new::ToDoNew};
use yew::prelude::*;
use yew_router::prelude::*;

mod constants;
mod pages;

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
        Route::ToDoShow { id } => html! { {id} },
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
