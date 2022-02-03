#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
  "Welcome to Pete K's Rocket/Rust app."
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Welcome, {}!", name)
}

#[get("/about")]
fn about() -> &'static str {
  "This is the about page of my Rocket/Rust app"
}

fn main() {
  rocket::ignite().mount("/", routes![index, about]).launch();
}
