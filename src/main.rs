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

#[derive_FromForm]
#[allow(dead_code)]
struct DetailsQuery {
    channel: String,
    fields: String
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

#[get("/snaps/details/<snap>?<details>")]
fn snap_details(snap: String, details: DetailsQuery) -> Content<Json> {
    let r = json!(
        {
            "anon_download_url": "https://api.snapcraft.io/api/v1/snaps/download/zn48fzkECo1HSsH9AOS8CAWNwO0lZVhK_3.snap",
            "architecture": ["amd64"],
            "binary_filesize": 1609728,
            "channel": "edge",
            "channel_maps_list": [
                {
                    "architecture": "amd64",
                    "map": [
                        {
                            "channel": "stable",
                            "info": null
                        },
                        {
                            "channel":
                            "candidate",
                            "info": null
                        },
                        {
                            "channel": "beta",
                            "info": null
                        },
                        {
                            "binary_filesize": 1609728,
                            "channel": "edge",
                            "confinement": "strict",
                            "epoch": "0",
                            "info": "released",
                            "revision": 3,
                            "version": "0+git.02bf65b"
                        }],
                    "track":"latest"
                }],
            "confinement":"strict",
            "content":"application",
            "deltas":[],
            "description": format!("Snap: {} Channel: {}", snap, details.channel),
            "developer_id":"ixIKmdMaUVa6JDwEaVTIIgQJOq9ghsjH",
            "download_sha3_384":"ec1ba04e87d8af40b617fb2668093c90171edca9834fef3c14ad91d26b8dc98f2b67a4dc0f049d2a0deb5d49abc886ce",
            "download_url":"https://api.snapcraft.io/api/v1/snaps/download/zn48fzkECo1HSsH9AOS8CAWNwO0lZVhK_3.snap",
            "epoch":"0",
            "icon_url": null,
            "last_updated":"2017-07-03T22:38:50.066730+00:00",
            "origin":"nsg",
            "package_name":"homer-nsg",
            "prices":{},
            "private":false,
            "publisher":"Stefan Berggren",
            "ratings_average":0.0,
            "revision":3,
            "screenshot_urls":[],
            "snap_id":"zn48fzkECo1HSsH9AOS8CAWNwO0lZVhK",
            "summary":"nsg's Home Automation API/System",
            "support_url":"",
            "title":"homer-nsg",
            "version":"0+git.02bf65b"
        }
    );

    Content(ContentType::new("application", "hal+json"), Json(r))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
               index,
               snap_metadata,
               snap_search,
               snap_details
        ])
        .launch();
}
