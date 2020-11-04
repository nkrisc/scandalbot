#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate base64;
use json_url;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use rocket::response::Redirect;
use rocket_contrib::{serve::StaticFiles, templates::Template};
use std::collections::HashMap;
use std::str;

#[derive(Deserialize, Serialize)]
struct Context {
    characters: Vec<Character>,
}

impl Context {
    pub fn with(characters: Vec<Character>) -> Context {
        Context {
            characters: characters,
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Character {
    #[serde(default)]
    id: String,
    #[serde(rename = "AD")]
    ad: i8,
    #[serde(rename = "FH")]
    fh: i8,
    #[serde(rename = "JY")]
    jy: i8,
    #[serde(rename = "KR")]
    kr: i8,
    #[serde(rename = "ML")]
    ml: i8,
    #[serde(rename = "SOB")]
    sob: i8,
    #[serde(rename = "SOTS")]
    sots: i8,
    #[serde(rename = "TD")]
    td: i8,
    #[serde(rename = "TOS")]
    tos: i8,
    #[serde(rename = "UR")]
    ur: i8,
    #[serde(rename = "WM")]
    wm: i8,
    #[serde(rename = "WS")]
    ws: i8,
    #[serde(rename = "NAME")]
    name: String,
    #[serde(rename = "WEEKLY_CHEST")]
    weekly_chest: i8,
    #[serde(rename = "RANK", default)]
    rank: i8,
    #[serde(default)]
    sim_score: f64,
}

#[get("/")]
fn index() -> Template {
    let context: HashMap<usize, usize> = HashMap::new();
    Template::render("index", context)
}

#[derive(Deserialize, Serialize)]
struct CompareValues(Vec<Character>);

#[get("/comparison?<values>")]
fn comparison(values: String) -> Result<Template, String> {
    match json_url::unpack(values) {
        Ok(unpacked) => Ok(Template::render("comparison", Context::with(unpacked))),
        Err(e) => Err(e.to_string()),
    }
}

#[get("/favicon.ico")]
fn favicon() -> Redirect {
    Redirect::to("/static/favicon.ico")
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, comparison, favicon])
        .mount(
            "/static",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/", "/static")),
        )
        .launch();
}
