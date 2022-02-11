#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::http::RawStr;

extern crate rust_integrator;

#[get("/")]
fn index() -> &'static str {
  "Welcome to my app, which calculates definite integrals.\n In the url bar type '/' followed by the lower limit of integration followed by '/' followed by the upper limit of integration followed by '/' followed by any function of x.  The function may be any algebraically legal combination of numbers, parentheses, and operations +, -, *, and/or **.  To represent division you must use either div, DIV, d, or D, because the usual division symbol ('/') has special meaning in a url.\n Example: To integrate the function 2x+3/(x^4+5) from 1 to 6, type 1/6/2x+3d(x**4+5) (for which the result should be 35.4136..)."
}

#[get("/<xi_str>/<xf_str>/<input_str>")]
fn evaluate(xi_str: &RawStr, xf_str: &RawStr, input_str: &RawStr) -> String {
  let mut fn_str = str::replace(input_str, " ", "");
  for stri in ["d", "div", "DIV", "D"] {
    fn_str = str::replace(&fn_str, stri, "/");
  }
  let expression = str::replace(&fn_str, "**", "^");
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
  let epsilon = (10_f64).powf(-12.);
  let mut dx = xf - xi;
  while (integral - integral_new).abs() > epsilon {
    integral = integral_new;
    integral_new = ptf.f * ptf.wt;
    let mut new_pts = vec![];
    dx /= 2.;
    for mut pt in pts {
      integral_new += pt.f * pt.wt;
      pt.wt = 1.;
      let x = pt.x + dx;
      let f = match rust_integrator::function(x, &expression) {
        Ok(f) => f,
        Err(msg) => return format!("Cannot evaluate function at {}: {}", xi, msg),
      };
      new_pts.append(&mut vec![pt, Pt{x, f, wt: 2.}]);
    }
    integral_new *= 4. * dx / 3.;
    pts = new_pts;
    pts[0].wt = 0.5;
  }
  format!("The integral from {} to {} of {} = {}", xi, xf, expression, integral_new)
}

fn main() {
  rocket::ignite().mount("/", routes![index, evaluate]).launch();
}
