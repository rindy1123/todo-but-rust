use fairing::db::DbMiddleware;
use rocket::{
    figment::providers::{Format, Toml},
    figment::Figment,
};

#[macro_use]
extern crate rocket;
mod controller;
mod fairing;
mod model;
mod view;

use controller::todos;

// TODO: docker, test
#[launch]
fn rocket() -> _ {
    let todo_api = routes![
        todos::index,
        todos::show,
        todos::create,
        todos::update,
        todos::delete
    ];
    let figment = Figment::from(rocket::Config::default()).merge(Toml::file("App.toml").nested());
    rocket::custom(figment)
        .mount("/", todo_api)
        .attach(DbMiddleware)
}
