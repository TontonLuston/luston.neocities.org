#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::fs::{FileServer, relative};
use rocket::http::uri::Origin;
use rocket::serde;
use rocket_dyn_templates::{Template, context};
use std::collections::HashMap;

#[derive(serde::Serialize)]
struct Site {
    title: String,
    subtitle: Option<String>,
    version: String,
    version_timestamp: String,
}

#[derive(serde::Serialize)]
struct Page {
    title: String,
    description: Option<String>,
    date: Option<String>,
    tags: Option<Vec<String>>,
    url: String,
    host: String,
}

fn now_function(
    _args: &HashMap<String, rocket_dyn_templates::tera::Value>,
) -> Result<rocket_dyn_templates::tera::Value, rocket_dyn_templates::tera::Error> {
    Ok(chrono::Utc::now().to_rfc3339().into())
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[catch(404)]
fn not_found(request: &rocket::Request<'_>) -> Template {
    let site = request.rocket().state::<Site>().unwrap();
    let uri = request.uri();
    let page = Page {
        title: "404 Not Found".to_string(),
        description: Some("The requested page does not exists or have been deleted.".to_string()),
        date: None,
        tags: None,
        url: uri.to_string(),
        host: request
            .headers()
            .get_one("Host")
            .unwrap_or("localhost")
            .to_string(),
    };

    Template::render(
        "404",
        context! {
            title: page.title.clone(),
            site: site,
            page: page,
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/assets", FileServer::from(relative!("/assets")))
        .mount("/", routes![index, hello])
        .register("/", catchers![not_found])
        .attach(AdHoc::on_ignite("Site Data", |rocket| async move {
            let site = Site {
                title: "luston.neocities.org".to_string(),
                subtitle: Some("A personal website".to_string()),
                version: format!("v{}", env!("CARGO_PKG_VERSION")),
                version_timestamp: chrono::Utc::now().to_rfc3339(),
            };

            rocket
                .attach(Template::custom(|engines| {
                    engines.tera.register_function("now", now_function);
                }))
                .manage(site)
        }))
}

#[get("/")]
fn index(
    site: &rocket::State<Site>,
    uri: &Origin<'_>,
    host: &rocket::http::uri::Host<'_>,
) -> Template {
    let page = Page {
        title: "Home".to_string(),
        description: Some("Welcome to my website".to_string()),
        date: None,
        tags: None,
        url: uri.to_string(),
        host: host.to_string(),
    };

    Template::render(
        "index",
        context! {
            title: page.title.clone(),
            site: site.inner(),
            page: page,
        },
    )
}
