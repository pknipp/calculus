#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::http::RawStr;
use evalexpr::*;

#[get("/")]
fn index() -> &'static str {
  "Welcome to Pete K's calculator app.\n In the url bar type any expression using +, -, *, or d (to represent division), and parentheses."
}

#[get("/<x_str>/<input_str>")]
fn evaluate(x_str: &RawStr, input_str: &RawStr) -> String {
  let mut fn_str = str::replace(input_str, "d", "/");
  // fn_str = str::replace(fn_str, "\*\*", "^");
  fn_str = str::replace(&fn_str.to_string(), "div", "/");
  fn_str = str::replace(&fn_str.to_string(), "DIV", "/");
  fn_str = str::replace(&fn_str.to_string(), "[dD]", "/");
  fn_str = str::replace(&fn_str.to_string(), " ", "");
  fn_str = str::replace(&fn_str.to_string(), "x", &format!("({})", x_str).to_string());
  let result = match eval(&fn_str.to_string()) {
    Ok(val) => format!("{}", val),
    Err(_) => String::from("String cannot be parsed."),
  };
  format!("{}\n{}", fn_str, result)
}

fn main() {
  rocket::ignite().mount("/", routes![index, evaluate]).launch();
}
