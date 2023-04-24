use fairing::db::DbMiddleware;
use rocket::{
    figment::providers::{Format, Toml},
    Config,
};

#[macro_use]
extern crate rocket;
mod controller;
mod fairing;
mod model;
mod view;

use controller::todos;

// TODO: test
#[launch]
fn rocket() -> _ {
    let todo_api = routes![
        todos::index,
        todos::show,
        todos::create,
        todos::update,
        todos::delete
    ];
    let figment = Config::figment().merge(Toml::file("App.toml").nested());
    rocket::custom(figment)
        .mount("/", todo_api)
        .attach(DbMiddleware)
}
