use rocket::{
    fairing::{Fairing, Info, Kind, Result},
    Build, Rocket,
};
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

pub struct DbMiddleware;

#[derive(Deserialize)]
struct DbConfig {
    namespace: String,
    database: String,
    username: String,
    password: String,
    host: String,
    port: String,
}

#[rocket::async_trait]
impl Fairing for DbMiddleware {
    fn info(&self) -> Info {
        Info {
            name: "DB Middleware",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result {
        let figment = rocket.figment().clone();
        let db_conf: DbConfig = figment.select("database").extract().unwrap();

        let db = Surreal::new::<Ws>(format!("{}:{}", db_conf.host, db_conf.port))
            .await
            .unwrap();
        db.signin(Root {
            username: &db_conf.username,
            password: &db_conf.password,
        })
        .await
        .unwrap();
        db.use_ns(db_conf.namespace)
            .use_db(db_conf.database)
            .await
            .unwrap();
        Ok(rocket.manage(db))
    }
}
