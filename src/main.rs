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
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

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

#[get("/file/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    print!("file: {:?}, files: {:?}", file, Path::new("/var/snapr/"));
    NamedFile::open(Path::new("/var/snapr/").join(file)).ok()
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
            "anon_download_url": "http://localhost:8000/file/hello-world.snap",
            "architecture": ["amd64"],
            "binary_filesize": 20480,
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
                            "binary_filesize": 20480,
                            "channel": "edge",
                            "confinement": "strict",
                            "epoch": "0",
                            "info": "released",
                            "revision": 27,                 // sync
                            "version": "0+git.02bf65b"
                        }],
                    "track":"latest"
                }],
            "confinement":"strict",
            "content":"application",
            "deltas":[],
            "description": format!("Snap: {} Channel: {}", snap, details.channel),
            "developer_id":"ixIKmdMaUVa6JDwEaVTIIgQJOq9ghsjH",
            "download_sha512":"717cb58bd9c55e421a9a02cab6cb836c26d59673598c76068a428a1b91c7215531ebd983da0e2fe708f867bc13b2986af1d6f71e00b7c9ba399ba7e333a59723",
            "download_url":"http://localhost:8000/file/hello-world.snap",
            "epoch":"0",
            "icon_url": null,
            "last_updated":"2017-07-03T22:38:50.066730+00:00",
            "origin":"nsg",
            "package_name":"hello-world",                   // sync
            "prices":{},
            "private":false,
            "publisher":"Stefan Berggren",
            "ratings_average":0.0,
            "revision":27,                                  // sync
            "screenshot_urls":[],
            "snap_id":"buPKUD3TKqCOgLEjjHx5kSiCpIs5cMuQ",   // sync
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
               snap_details,
               files
        ])
        .launch();
}
