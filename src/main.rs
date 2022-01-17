#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;

use dotenv::dotenv;
use rocket_codegen::{catchers, routes};

use routes::{books_routes, catch_routes, people_routes};

mod books;
mod db;
mod people;
mod routes;
mod schema;

fn rocket() -> rocket::Rocket {
    dotenv().ok();
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let pool = db::init_pool(database_url);

    rocket::ignite()
        .manage(pool)
        .mount(
            "/api-v1",
            routes![
                people_routes::index,
                people_routes::show,
                people_routes::country,
                people_routes::new,
                people_routes::update,
                people_routes::update_name,
                people_routes::delete,
            ],
        )
        .mount(
            "/api-v1",
            routes![
                books_routes::index,
                books_routes::show,
                books_routes::author,
                books_routes::new,
                books_routes::update,
                books_routes::update_published,
                books_routes::delete,
            ],
        )
        .register(catchers![catch_routes::not_found])
}

fn main() {
    rocket().launch();
}
