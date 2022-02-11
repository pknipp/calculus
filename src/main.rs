#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::http::RawStr;

extern crate rust_integrator;

#[get("/")]
fn index() -> &'static str {
  "Welcome to Pete K's calculator app.\n In the url bar type '/' followed by any number followed by '/' followed by any function of x.  The function is any algebraically legal combination of numbers, parentheses, and operations +, -, *, and/or **.  To represent division you must use either div, DIV, d, or D, because the usual division symbol ('/') has special meaning in a url.\n
  Example: To evaluate the function 2x+3/x^4 when x=5, type 5/2x+3dx**4 (for which the result should be 10.0048)."
}

// impl Pt {
// struct Pt {
  // x: f64,
  // f: f64,
  // wt: f64,
// }
// }

#[get("/<xi_str>/<xf_str>/<input_str>")]
fn evaluate(xi_str: &RawStr, xf_str: &RawStr, input_str: &RawStr) -> String {
  let mut fn_str = str::replace(input_str, " ", "");
  fn_str = str::replace(&fn_str, "**", "^");
  for stri in ["d", "div", "DIV", "D"] {
    fn_str = str::replace(&fn_str, stri, "/");
  }
  struct Pt {
    x: f64,
    f: f64,
    wt: f64,
  }
  let xi = match xi_str.parse() {
    Ok(x) => x,
    Err(message) => return format!("{} cannot be converted to float: {}", xi_str, message),
  };
  let xf = match xf_str.parse() {
    Ok(x) => x,
    Err(message) => return format!("{} cannot be converted to float: {}", xf_str, message),
  };
  let fi = match rust_integrator::function(xi, &expression) {
    Ok(f) => f,
    Err(message) => return format!("Cannot evaluate function at {}: {}", xi, message),
  };
  let ff = match rust_integrator::function(xf, &expression) {
    Ok(f) => f,
    Err(message) => return format!("Cannot evaluate function at {}: {}", xi, message),
  };
  let mut pts = vec![Pt{x: xi, f: fi, wt: 0.5}];
  let ptf = Pt{x: xf, f: ff, wt: 0.5};
  let mut integral = f64::INFINITY;
  let mut integral_new = 0.;
  let epsilon = 0.01;
  let mut dx = xf - xi;
  while (integral - integral_new).abs() > epsilon {
    integral = integral_new;
    integral_new = ptf.f * ptf.wt;
    let new_pts = vec![];
    dx /= 2.;
    for pt in pts {
      integral_new += pt.f * pt.wt;
      pt.wt = 1.;
      let x = pt.x + dx;
      let f = match rust_integrator::function(x, &expression) {
        Ok(f) => f,
        Err(f) => return format!("Cannot evaluate function at {}: {}", xi, message),
      };
      new_pts.push(Pt{x, f, wt: 2.});
    }
    integral_new *= (2. * dx);
    pts[0].wt = 0.5;
    pts.append(&mut new_pts);
  }
  format!("integral = {}", integral_new)
}

fn main() {
  rocket::ignite().mount("/", routes![index, evaluate]).launch();
}
