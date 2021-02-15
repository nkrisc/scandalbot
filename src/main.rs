#![feature(proc_macro_hygiene, decl_macro)]

extern crate base64;
#[macro_use]
extern crate rocket;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use json_url;
use rocket::response::Redirect;
use rocket_contrib::{serve::StaticFiles, templates::Template};
use std::{collections::HashMap, str};

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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct Character {
    #[serde(default)]
    id: String,
    #[serde(rename = "HOA")]
    hoa: i8,
    #[serde(rename = "MOTS")]
    mots: i8,
    #[serde(rename = "DOS")]
    dos: i8,
    #[serde(rename = "NW")]
    nw: i8,
    #[serde(rename = "PF")]
    pf: i8,
    #[serde(rename = "SD")]
    sd: i8,
    #[serde(rename = "SOA")]
    soa: i8,
    #[serde(rename = "TOP")]
    top: i8,
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
