#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::http::RawStr;
use rocket::response::content;

extern crate calculus;
extern crate serde_json;

#[get("/")]
fn index() -> content::Html<String> {
  content::Html(calculus::general_page())
}

#[get("/differentiation")]
fn differentiation() -> content::Html<String> {
  content::Html(calculus::differentiation_page())
}

#[get("/integration")]
fn integration() -> content::Html<String> {
  content::Html(calculus::integration_page())
}

#[get("/root-finding")]
fn root_finding() -> content::Html<String> {
  content::Html(calculus::root_finding_page())
}

#[get("/differentiation/json/<x_str>/<input_str>")]
fn differentiate_json(x_str: &RawStr, input_str: &RawStr) -> String {
  match calculus::differentiate_raw(x_str, input_str) {
    Ok(results) => serde_json::to_string(&results).unwrap(),
    Err(message) => format!("{{\"message\": {}}}", message),
  }
}

#[get("/integration/json/<xi_str>/<xf_str>/<input_str>")]
fn integrate_json(xi_str: &RawStr, xf_str: &RawStr, input_str: &RawStr) -> String {
  match calculus::integrate_raw(xi_str, xf_str, input_str) {
    Ok(results) => serde_json::to_string(&results).unwrap(),
    Err(message) => format!("{{\"message\": {}}}", message),
  }
}

#[get("/differentiation/<x_str>/<input_str>")]
fn differentiate(x_str: &RawStr, input_str: &RawStr) -> content::Html<String> {
  let results = match calculus::differentiate_raw(x_str, input_str) {
    Ok(results) => results,
    Err(message) => return content::Html(format!("{}<br><br><b>result</b> for the function f(x) = {}:<br>{}",
      calculus::differentiation_page(),
      input_str,
      message
    )),
  };
  let text = if results.nonsingular {""} else {"<br>(The function does not exist at that point, but these are the limits.)"};
  let mut expression = input_str.to_string();
  expression = str::replace(&expression, "%5E", "^");
	expression = str::replace(&expression, "%20", ""); // %20 is url encoding of space
  for stri in ["div", "DIV", "d", "D"] {
    expression = str::replace(&expression, stri, "/"); // division operation is a special URL char
  }
  content::Html(format!("{}<br><br><b>results</b> at x = {} for the function f(x) = {}:{}<ul><li>f = {}</li><li>f' = {}</li><li>f'' = {}</li><li>f''' = {}</li></ul>",
    calculus::differentiation_page(),
    results.x,
    expression,
    text,
    results.derivs[0],
    results.derivs[1],
    results.derivs[2],
    results.derivs[3],
  ))
}

#[get("/integration/<xi_str>/<xf_str>/<input_str>")]
fn integrate(xi_str: &RawStr, xf_str: &RawStr, input_str: &RawStr) -> content::Html<String> {
  let results = match calculus::integrate_raw(xi_str, xf_str, input_str) {
    Ok(results) => results,
    Err(message) => return content::Html(format!("{}<br><br><b>result</b> for the integral from x = {} to x = {} of the function f(x) = {}:<br>{}",
      calculus::differentiation_page(),
      xi_str,
      xf_str,
      input_str,
      message
    )),
  };
  let mut expression = input_str.to_string();
  expression = str::replace(&expression, "%5E", "^");
	expression = str::replace(&expression, "%20", ""); // %20 is url encoding of space
  for stri in ["div", "DIV", "d", "D"] {
    expression = str::replace(&expression, stri, "/"); // division operation is a special URL char
  }
  content::Html(format!("{}<br><br><b>result</b>: {} equals the definite integral from x = {} to x = {} of the function f(x) = {}.<br>Convergence to an absolute accuracy of {} required {} subdivisions.",
    calculus::integration_page(),
    results.integral,
    results.xi,
    results.xf,
    str::replace(&expression, "X", "x"),
    results.epsilon,
    results.subdivisions,
  ))
}

#[get("/root-finding/<xi_str>/<input_str>")]
fn find_root(xi_str: &RawStr, input_str: &RawStr) -> content::Html<String> {
  let result = match calculus::find_root_raw(xi_str, input_str) {
    Ok(result) => result,
    Err(message) => return content::Html(format!("{}<br><br><b>result</b> for finding a root of the function f(x) = {} after starting at x = {}:<br>{}",
      calculus::differentiation_page(),
      input_str,
      xi_str,
      message
    )),
  };
  let mut expression = input_str.to_string();
  expression = str::replace(&expression, "%5E", "^");
	expression = str::replace(&expression, "%20", ""); // %20 is url encoding of space
  for stri in ["div", "DIV", "d", "D"] {
    expression = str::replace(&expression, stri, "/"); // division operation is a special URL char
  }
  content::Html(format!("{}<br><br><b>result</b>: {} is the root of the function f(x) = {} which is found after starting from x = {}.<br>Convergence to an absolute accuracy of {} required {} steps.",
    calculus::root_finding_page(),
    result.x,
    str::replace(&expression, "X", "x"),
    result.xi,
    result.epsilon,
    result.steps,
  ))
}

fn main() {
  rocket::ignite().mount("/", routes![index, differentiation, integration, root_finding, differentiate, differentiate_json, integrate, integrate_json, find_root]).launch();
}
