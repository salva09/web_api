#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Message {
    contents: String,
}

#[put("/", data = "<msg>")]
fn update(msg: Json<Message>) -> JsonValue {
    println!("{}", msg.0.contents);
    json!({ "message": msg.0.contents })
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    rocket::ignite().mount("/", routes![update, hello]).launch();
}
