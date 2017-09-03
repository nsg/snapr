#![feature(plugin)]
#![plugin(rocket_codegen)]
#![recursion_limit="128"]

extern crate rocket;
// extern crate serde_json;
// extern crate curl;

#[macro_use] extern crate rocket_contrib;

use rocket_contrib::Json;
use rocket::response::content::Content;
use rocket::http::ContentType;


#[derive_FromForm]
#[allow(dead_code)]
struct SearchQuery {
    confinement: String,
    fields: String,
    section: Option<String>,
    q: String
}

#[get("/")]
fn index() {
    print!("index!")
}

#[post("/snaps/metadata", format = "application/json", data = "<body>")]
fn snap_metadata(body: String) -> Json {
    print!("metadata request, got: {}", body);
    Json(json!({})) // Dummy response
}

#[get("/snaps/search?<search>")]
fn snap_search(search: SearchQuery) -> Content<Json> {
    let r = json!({

        "_embedded": {
            "clickindex:package": [
            {
                "publisher": "Canonical",
                "ratings_average": 0,
                "name": "xkcd-webserver.canonical",
                "package_name": format!("{}", search.q),
                "title": "xkcd-webserver",
                "icon_url": "https://myapps.developer.ubuntu.com/site_media/appmedia/2015/03/xkcd.svg.png",
                "price": 0,
                "summary": "Tänka sig vad man kan åstakomma med lite Rust",
                "content": "application",
                "alias": "xkcd-webserver",
                "version": "16.04-6",
                "_links": {
                "self": {
                    "href": "https://search.apps.ubuntu.com/api/v1/package/EQPfyVOJF0AZNz9P2IJ6UKwldLFN5TzS"
                }
                },
                "prices": {},
                "release": [
                "16"
                ],
                "plugs": [
                "network",
                "network-bind"
                ],
                "snap_id": "EQPfyVOJF0AZNz9P2IJ6UKwldLFN5TzS",
                "slots": [],
                "revision": 16
            }
            ]
        }
        });

    Content(ContentType::new("application", "hal+json"), Json(r))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
               index,
               snap_metadata,
               snap_search
        ])
        .launch();
}
