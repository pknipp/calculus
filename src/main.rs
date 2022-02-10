#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::http::RawStr;
// use evalexpr::*;

extern crate function;

#[get("/")]
fn index() -> &'static str {
  "Welcome to Pete K's calculator app.\n In the url bar type any expression using +, -, *, or d (to represent division), and parentheses."
}

#[get("/<x_str>/<input_str>")]
fn evaluate(x_str: &RawStr, input_str: &RawStr) -> String {
  let mut fn_str = str::replace(input_str, " ", "");
  fn_str = str::replace(&fn_str, "**", "^");
  for stri in ["d", "div", "DIV", "D"] {
    fn_str = str::replace(&fn_str, stri, "/");
  }
  let expression = str::replace(&fn_str.to_string(), "x", &format!("({})", x_str).to_string());
  let expression_copy = format!("{}", &expression);
  let result = match function::parse_expression(expression) {
    Ok(val) => format!("{}", val),
    Err(message) => format!("{}", message),
  };
  format!("input string: {}\n function: f(x) = {}\n result: f({}) = {} = {}", input_str, fn_str, x_str, expression_copy, result)
}

fn main() {
  rocket::ignite().mount("/", routes![index, evaluate]).launch();
}
