use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;

mod constant;
mod controller;
mod core;
mod handler;
mod utils;

#[launch]
fn rocket() -> _ {
    load_config!();
    rocket::build()
        .attach(Template::fairing())
        .mount("/", handler::register_routes())
}
