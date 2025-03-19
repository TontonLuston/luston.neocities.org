#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::{Template, context};

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello])
        .mount("/assets", FileServer::from(relative!("/assets")))
        .attach(Template::fairing())
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            title: "luston.neocities.org",
        },
    )
}
