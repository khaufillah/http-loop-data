use crate::warn;
use rocket::{
    fs::{relative, NamedFile},
    http::Status,
    response::content,
    Route,
};
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
struct Health {
    health: String,
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            version: env::var("VERSION").ok(),
            base_url: env::var("BASE_URL").ok(),
        },
    )
}

#[get("/static")]
async fn static_redirect() -> Status {
    Status::NotFound
}

#[get("/static/<path..>")]
async fn static_file(path: PathBuf) -> Result<NamedFile, Status> {
    let path = Path::new(relative!("src/resources/static")).join(path);
    NamedFile::open(path).await.map_err(|err| {
        warn!(err);
        Status::NotFound
    })
}

#[get("/api")]
fn api() -> content::RawJson<String> {
    let health = Health {
        health: String::from("ok"),
    };
    // Serialize the health to JSON
    let json = serde_json::to_string(&health).unwrap();
    content::RawJson(json)
}

pub fn register_routes() -> Vec<Route> {
    routes![index, static_redirect, static_file, api]
}
