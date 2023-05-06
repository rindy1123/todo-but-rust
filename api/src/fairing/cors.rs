use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response,
};
use serde::Deserialize;

use crate::rocket;

pub struct Cors;

#[derive(Deserialize)]
struct CorsSetting {
    origin: String,
}

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "CORS",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _: &'r Request<'_>, response: &mut Response<'r>) {
        let figment = rocket().figment().clone();
        let setting: CorsSetting = figment.select("cors").extract().unwrap();

        response.set_header(Header::new("Access-Control-Allow-Origin", setting.origin));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, PUT, OPTIONS, DELETE",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    }
}
