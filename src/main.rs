#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::http::RawStr;

extern crate rust_integrator;

#[get("/")]
fn index() -> &'static str { rust_integrator::INSTRUCTIONS }

#[get("/<x_str>/<input_str>")]
fn differentiate(x_str: &RawStr, input_str: &RawStr) -> String {
  let matrix = vec![
    vec![-2., 8., 8., -2.],
    vec![-1., 8., -8., 1.],
    vec![2., -2., -2., 2.],
    vec![1., -2., 2., -1.]
  ];
  let x = match rust_integrator::parse_expression(x_str.to_string()) {
    Ok(x) => x,
    Err(message) => return format!("{} cannot be converted to float: {}", x_str, message),
  };
  let _f = match rust_integrator::function(x, input_str) {
    Ok(f) => f,
    Err(message) => return message,
  };
  let dx = 0.00001;
  let steps = vec![2., 1., -1., -2.];
  let mut fs = vec![];
  for step in steps {
    fs.push(match rust_integrator::function(x + step * dx, input_str) {
      Ok(f) => f,
      Err(message) => return message,
    });
  }

  let mut derivs = vec![];
  for row in matrix {
    let mut deriv = 0.;
    for (i, element) in row.iter().enumerate() {
      deriv += element * fs[i] / 12.;
    }
    derivs.push(deriv);
  }
  format!("{}At x = {}, the value and first three derivatives of the function {} equal \n{}, \n{}, \n{}, and \n{},\nrespectively", rust_integrator::INSTRUCTIONS, x, input_str, derivs[0], derivs[1] / dx, 2. * derivs[2] / dx / dx, 6. * derivs[3] / dx / dx / dx)
}

#[get("/<xi_str>/<xf_str>/<input_str>")]
fn integrate(xi_str: &RawStr, xf_str: &RawStr, input_str: &RawStr) -> String {
  struct Pt {
    x: f64,
    f: f64,
    wt: f64,
  }
  let mut pts = vec![];
  for x_str in &[xi_str, xf_str] {
    let x = match rust_integrator::parse_expression(x_str.to_string()) {
      Ok(x) => x,
      Err(message) => return format!("{} cannot be converted to float: {}", x_str, message),
    };
    let f = match rust_integrator::function(x, input_str) {
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
  let mut number = 1;
  while !aitkens.is_finite() || !aitkens_new.is_finite() || (aitkens_new - aitkens).abs() > epsilon {
    number *= 2;
    let mut integral_new = ptf.f * ptf.wt;
    let mut new_pts = vec![];
    dx /= 2.; // start preparing next set of integration points
    for mut pt in pts {
      integral_new += pt.f * pt.wt;
      pt.wt = 1.; // wt for most points is 1 except for their first appearance
      let x = pt.x + dx; // x-coord of next point
      let f = match rust_integrator::function(x, input_str) {
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
  let mut expression = input_str.to_string();
  for stri in ["d", "div", "DIV", "D"] {
    expression = str::replace(&expression, stri, "/"); // division operation is a special URL char
  }
  format!("{}{}{} equals the integral of {} from {} to {}.\nConvergence to an absolute accuracy of {} required {} subdivisions.", rust_integrator::INSTRUCTIONS, "RESULTS:\n", aitkens_new, str::replace(&expression, "X", "x")
  , pts[0].x, ptf.x, epsilon, number)
}

fn main() {
  rocket::ignite().mount("/", routes![index, integrate, differentiate]).launch();
}
