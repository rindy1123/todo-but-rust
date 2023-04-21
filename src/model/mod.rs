use rocket::State;
use surrealdb::{engine::remote::ws::Client, Surreal};

pub mod todo;

pub type DB = State<Surreal<Client>>;
