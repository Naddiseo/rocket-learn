#![feature(plugin, custom_derive, rust_2018_preview)]
#![plugin(rocket_codegen)]


#[macro_use]
extern crate lazy_static;

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;
extern crate tera;
//extern crate r2d2;

use rocket_contrib::Template;
use tera::{Context};
use dotenv::dotenv;


mod db;
mod forms;
mod fields;

mod static_files;
mod auth;
mod home;

#[get("/")]
fn index() -> Template {
    let context = Context::new();
    Template::render("index", &context)
}

fn main() {
    dotenv().ok();
    
    rocket::ignite()
    .manage(db::init_pool())
    .mount("/", routes![
        index,
        auth::login,
        auth::logout,
        
        home::home,
        
        static_files::all,
    ])
    .attach(Template::fairing())
    .launch();
}
