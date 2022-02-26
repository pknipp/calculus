#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::http::RawStr;
use rocket::response::content;

extern crate calculus;

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

#[get("/differentiation/<x_str>/<input_str>")]
fn differentiate(x_str: &RawStr, input_str: &RawStr) -> content::Html<String> {
  let x = match calculus::parse_expression(x_str.to_string()) {
    Ok(x) => x,
    Err(message) => return content::Html(format!("{}<br><br><b>result</b> for the function f(x) = {}:<br>{}",
      calculus::differentiation_page(),
      input_str,
      message
    )),
  };
  // let expression = calculus::preparse(input_str);
  let f = calculus::function(x, &input_str);
  let dx = 0.001;
  let steps = vec![2., 1., -1., -2.];
  let mut fs = vec![];
  for step in steps {
    fs.push(match calculus::function(x + step * dx, &input_str) {
      Ok(f) => f,
      Err(message) => return content::Html(format!("{}<br><br><b>result</b> at x = {}:<br>{}",
        calculus::differentiation_page(),
        x_str,
        message,
      )),
    });
  }
  let mut f0 = 0.;
  let nonsingular = f.is_ok();
  if nonsingular {
    f0 = f.unwrap();
  }
  let derivs = vec![
    // How to use values at discrete points to calculate function and derivative values
    // For every other case, allowance needs to be made for a removable singularity.
    if nonsingular {f0} else {(fs[1] + fs[2]) / 2.},
    (fs[1] - fs[2]) / 2. / dx,
    if nonsingular {(fs[1] - 2. * f0 + fs[2]) / dx / dx} else {(fs[0] - fs[1] - fs[2] + fs[3]) / 3. / dx / dx},
    (fs[0] - fs[3] - 2. * fs[1] + 2. * fs[2]) / 2. / dx / dx / dx,
  ];
  let text = if nonsingular {""} else {"<br>(The function does not exist at that point, but these are the limits."};
  let mut expression = input_str.to_string();
  expression = str::replace(&expression, "%5E", "^");
	expression = str::replace(&expression, "%20", ""); // %20 is url encoding of space
  for stri in ["div", "DIV", "d", "D"] {
    expression = str::replace(&expression, stri, "/"); // division operation is a special URL char
  }
  content::Html(format!("{}<br><br><b>results</b> at x = {} for the function f(x) = {}:{}<ul><li>f = {}</li><li>f' = {}</li><li>f'' = {}</li><li>f''' = {}</li></ul>",
    calculus::differentiation_page(),
    x,
    expression,
    text,
    derivs[0],
    derivs[1],
    derivs[2],
    derivs[3],
  ))
}

#[get("/integration/<xi_str>/<xf_str>/<input_str>")]
fn integrate(xi_str: &RawStr, xf_str: &RawStr, input_str: &RawStr) -> content::Html<String> {
  struct Pt {
    x: f64,
    f: f64,
    wt: f64,
  }
  let mut pts = vec![];
  for x_str in &[xi_str, xf_str] {
    let x = match calculus::parse_expression(x_str.to_string()) {
      Ok(x) => x,
      Err(message) => return content::Html(format!("{}<br><br><b>result</b> for integral of the function f(x) = {}:<br>{}",
        calculus::integration_page(),
        input_str,
        message,
      )),
    };
    let f = match calculus::function(x, input_str) {
      Ok(f) => f,
      Err(message) => return content::Html(format!("{}<br><br><b>result</b> for integration of the function f(x) = {}:<br>{}",
        calculus::integration_page(),
        input_str,
        message,
      )),
    };
    pts.push(Pt{x, f, wt: 0.5}); // non-0th pt will only reside in vector for an instant
  }
  let ptf = match pts.pop() { // final point will be handled separately, going forward
    Some(ptf) => ptf,
    None => return content::Html(format!("{}<br><br><b>result</b> for integration of the function <i>f</i> = {}:<br>{}",
      calculus::integration_page(),
      input_str,
      "Missing integration endpoint".to_string(),
    )),
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
      let f = match calculus::function(x, input_str) {
        Ok(f) => f,
        Err(msg) => return content::Html(format!("Cannot evaluate function at x: {}{}", pt.x, msg)),
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
  expression = str::replace(&expression, "%5E", "^");
	expression = str::replace(&expression, "%20", ""); // %20 is url encoding of space
  for stri in ["div", "DIV", "d", "D"] {
    expression = str::replace(&expression, stri, "/"); // division operation is a special URL char
  }
  content::Html(format!("{}<br><br><b>result</b>: {} equals the definite integral from x = {} to x = {} of the function f(x) = {}.<br>Convergence to an absolute accuracy of {} required {} subdivisions.",
    calculus::integration_page(),
    aitkens_new,
    pts[0].x,
    ptf.x,
    str::replace(&expression, "X", "x"),
    epsilon,
    number,
  ))
}

fn main() {
  rocket::ignite().mount("/", routes![index, differentiation, integration, differentiate, integrate]).launch();
}
