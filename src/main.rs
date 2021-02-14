#![feature(proc_macro_hygiene, decl_macro)]
use rocket::request::Form;
use rocket::response::{Flash, Redirect};
use rocket::*;

#[derive(FromForm)]
struct Task {
    description: String,
    completed: bool,
}

#[post("/", data = "<task>")]
fn new(task: Form<Task>) -> Flash<Redirect> {
    if task.description.is_empty() {
        Flash::error(Redirect::to("/"), "Cannot be empty.")
    } else {
        Flash::success(Redirect::to("/"), "Task added.")
    }
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    rocket::ignite().mount("/", routes![new, hello]).launch();
}
