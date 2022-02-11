#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::http::RawStr;

extern crate rust_integrator;

#[get("/")]
fn index() -> &'static str {
  "Welcome to Pete K's calculator app.\n In the url bar type '/' followed by any number followed by '/' followed by any function of x.  The function is any algebraically legal combination of numbers, parentheses, and operations +, -, *, and/or **.  To represent division you must use either div, DIV, d, or D, because the usual division symbol ('/') has special meaning in a url.\n
  Example: To evaluate the function 2x+3/x^4 when x=5, type 5/2x+3dx**4 (for which the result should be 10.0048)."
}

#[get("/<_xi_str>/<_xf_str>/<input_str>")]
fn evaluate(_xi_str: &RawStr, _xf_str: &RawStr, input_str: &RawStr) -> String {
  let mut fn_str = str::replace(input_str, " ", "");
  fn_str = str::replace(&fn_str, "**", "^");
  for stri in ["d", "div", "DIV", "D"] {
    fn_str = str::replace(&fn_str, stri, "/");
  }
  // let expression = str::replace(&fn_str, "x", &format!("({})", x_str));
  // let expression_copy = &expression.to_string();
  // let result = match rust_integrator::parse_expression(expression) {
    // Ok(val) => format!("{}", val),
    // Err(message) => message,
  // };
  // format!("input string: {}\n function: f(x) = {}\n result: f({}) = {} = {}", input_str, fn_str, x_str, expression_copy, result)
  fn_str
}

fn main() {
  rocket::ignite().mount("/", routes![index, evaluate]).launch();
}
