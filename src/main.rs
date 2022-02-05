#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::http::RawStr;
use evalexpr::*;

#[get("/")]
fn index() -> &'static str {
  "Welcome to Pete K's calculator app.\n In the url bar type any expression using +, -, *, or d (to represent division), and parentheses."
}

#[get("/<input_str>")]
fn evaluate(input_str: &RawStr) -> String {
  let math_str = str::replace(input_str, "d", "/");
  match eval(&math_str.to_string()) {
    Ok(val) => format!("{}", val),
    Err(_) => String::from("String cannot be parsed."),
  }
}

fn main() {
  rocket::ignite().mount("/", routes![index, evaluate]).launch();
}
