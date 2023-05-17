use fairing::{cors::Cors, db::DbMiddleware};
use rocket::{
    figment::providers::{Format, Toml},
    http::Status,
    Config,
};

#[macro_use]
extern crate rocket;
mod controller;
mod fairing;
mod model;
mod view;

use controller::todos;

#[options("/<_..>")]
fn cors_handler() -> Status {
    Status::NoContent
}
#[launch]
fn rocket() -> _ {
    let todo_api = routes![
        todos::index,
        todos::show,
        todos::create,
        todos::update,
        todos::delete,
        cors_handler
    ];
    let figment = Config::figment().merge(Toml::file("App.toml").nested());
    rocket::custom(figment)
        .mount("/", todo_api)
        .attach(DbMiddleware)
        .attach(Cors)
}
