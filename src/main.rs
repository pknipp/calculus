#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::http::RawStr;

extern crate rust_integrator;

#[get("/")]
fn index() -> &'static str {
  "Welcome to my app, which calculates definite integrals.\n In the url bar type '/' followed by the lower limit of integration followed by '/' followed by the upper limit of integration followed by '/' followed by any function of x.  The function may be any algebraically legal combination of x, numbers, parentheses, and operations +, -, *, ** (or ^) and/or the most common unary functions: abs, acos, acosh, acot, acoth, acsc, acsch, asec, asech, asin, asinh, atan, atanh, cbrt, ceil, cos, cot, csc, exp, floor, ln, log10, log2, sec, sin, sqrt, tan, and trunc.  (See https://doc.rust-lang.org/std/primitive.f64.html for more information about any of these functions.) To represent division you must use either div, DIV, d, or D, because the usual division symbol ('/') has special meaning in a url.\n Example: To integrate the function 2x+3/(x^4+5) from 1 to 6, type 1/6/2x+3d(x**4+5) (for which the result should be 35.4136..)."
}

#[get("/<xi_str>/<xf_str>/<input_str>")]
fn integrate(xi_str: &RawStr, xf_str: &RawStr, input_str: &RawStr) -> String {
  let mut fn_str = str::replace(&input_str.to_lowercase(), " ", ""); // simplify parsing
  for stri in ["d", "div", "DIV", "D"] {
    fn_str = str::replace(&fn_str, stri, "/"); // division operation is a special URL char
  }
  // following 3-line swap prevents confusion between "x" as var and char in "exp" function.
  fn_str = str::replace(&fn_str, "exp", "EXP");
  fn_str = str::replace(&fn_str, "x", "X");
  fn_str = str::replace(&fn_str, "EXP", "exp");
  let expression = str::replace(&fn_str, "**", "^"); // in case user chooses ^ instead of **
  struct Pt {
    x: f64,
    f: f64,
    wt: f64,
  }
  let mut pts = vec![];
  for x_str in &[xi_str, xf_str] {
    let x = match x_str.parse() {
      Ok(x) => x,
      Err(message) => return format!("{} cannot be converted to float: {}", x_str, message),
    };
    let f = match rust_integrator::function(x, &expression) {
      Ok(f) => f,
      Err(message) => return message,
    };
    pts.push(Pt{x, f, wt: 0.5}); // non-0th pt will only reside in vector for an instant
  }
  let ptf = match pts.pop() { // final point will be handled separately, going forward
    Some(ptf) => ptf,
    None => return "Missing integration endpoint".to_string(),
  };
  let mut integral = f64::INFINITY;
  // variables needed to implement Aitken's algo to accelerate a geometric sequence
  let mut aitkens = f64::INFINITY;
  let mut aitkens_new = f64::INFINITY;
  let epsilon = (10_f64).powf(-12.);
  let mut dx = ptf.x - pts[0].x; // interval for Simpson's rule
  while !aitkens.is_finite() || !aitkens_new.is_finite() || (aitkens_new - aitkens).abs() > epsilon {
    let mut integral_new = ptf.f * ptf.wt;
    let mut new_pts = vec![];
    dx /= 2.; // start preparing next set of integration points
    for mut pt in pts {
      integral_new += pt.f * pt.wt;
      pt.wt = 1.; // wt for most points is 1 except for their first appearance
      let x = pt.x + dx; // x-coord of next point
      let f = match rust_integrator::function(x, &expression) {
        Ok(f) => f,
        Err(msg) => return format!("Cannot evaluate function at {}: {}", pt.x, msg),
      };
      new_pts.append(&mut vec![pt, Pt{x, f, wt: 2.}]);
    }
    integral_new *= 4. * dx / 3.; // overall factor, for extended Simpson's rule
    pts = new_pts; // Overwrite pts vector, which was moved during iteration
    pts[0].wt = 0.5; // wt of 0th and last points is always 0.5 (ie, never 1.)
    aitkens = aitkens_new;
    aitkens_new = integral_new;
    if integral.is_finite() {
      // Aitken's correction, because integral's accuracy is O(dx^4)
      aitkens_new += (integral_new - integral ) / (16. - 1.);
    }
    integral = integral_new;
  }
  format!("{} equals the integral of {} from {} to {}.", aitkens_new, str::replace(&expression, "X", "x")
  , pts[0].x, ptf.x)
}

fn main() {
  rocket::ignite().mount("/", routes![index, integrate]).launch();
}
