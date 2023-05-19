use rocket::{
    fairing::{Fairing, Info, Kind, Result},
    figment::{
        providers::{Format, Toml},
        Figment,
    },
    Build, Rocket,
};
use serde::Deserialize;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use uuid::Uuid;

pub type DB = Surreal<Client>;

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

pub struct TestDbMiddleware;

#[derive(Deserialize)]
struct TestDbConfig {
    namespace: String,
    username: String,
    password: String,
    host: String,
    port: String,
}

#[rocket::async_trait]
impl Fairing for TestDbMiddleware {
    fn info(&self) -> Info {
        Info {
            name: "DB Middleware For Testing",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result {
        let db = TestDbMiddleware::setup_db().await;
        Ok(rocket.manage(db))
    }
}

impl TestDbMiddleware {
    pub async fn setup_db() -> DB {
        let figment = Figment::new().merge(Toml::file("App.toml").nested());
        let db_conf: TestDbConfig = figment.select("test_database").extract().unwrap();
        let db = Surreal::new::<Ws>(format!("{}:{}", db_conf.host, db_conf.port))
            .await
            .unwrap();
        db.signin(Root {
            username: &db_conf.username,
            password: &db_conf.password,
        })
        .await
        .unwrap();
        let database = Uuid::new_v4().to_string();
        db.use_ns(db_conf.namespace).use_db(database).await.unwrap();
        db
    }
}
