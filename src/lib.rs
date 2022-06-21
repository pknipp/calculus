use std::f64::consts::PI;
use rocket::http::RawStr;
use serde::{Serialize, Deserialize};

mod helper;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    person_id: i32,
    person_name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    user_id: i32,
    user_name: String,
    user_password: String,
    user_person: Person
}

pub const INSTRUCTIONS: &str = "WELCOME TO MY CALCULUS APP";

const FUNCTION: &str = "The function may be any algebraically legal combination of the variable letter(s), numbers, parentheses, and/or binary operations +, -, *, ** (encouraged) or ^ (discouraged), PI and/or the most common unary functions: <tt>abs, acos, acosh, acot, acoth, acsc, acsch, asec, asech, asin, asinh, atan, atanh, cbrt, ceil, cos, cot, csc, exp, exp2, exp_m1, floor, fract, ln, ln_1p, log10, log2, round, sec, signum, sin, sqrt, tan, and trunc</tt>.  (See <a href='https://doc.rust-lang.org/std/primitive.f64.html'>docs</a> for more information.) To represent division you must use either <tt>div, DIV, d, or D</tt> because the usual division symbol (<tt>/</tt>) has special meaning in a url.  Implied multiplication is allowed.  Spaces are allowed but discouraged.";

const NOTE1: &str = "The construction rules for the values of any variable in the url";
const NOTE2: &str = " are the same as those for the function except - of course - it cannot include the letter which represents the variable.";

struct Link<'a> {
	url: &'a str,
	inner: &'a str,
	outer: &'a str,
}

pub struct LongPage {
	title: String,
	links: String,
	instructions: String,
	note: String,
	example: String,
	algorithm: String,
	json: String,
}

const LINKS: [Link; 8] = [
	Link{
		url: "https://pknipp.github.io/math",
		inner: "back to",
		outer: " math APIs page",
	},
	Link{
		url: "https://basic-calculus.herokuapp.com",
		inner: "back to",
		outer: " calculus page",
	},
	Link{
		url: "https://basic-calculus.herokuapp.com/differentiation",
		inner: "differentiation",
		outer: "",
	},
	Link{
		url: "https://basic-calculus.herokuapp.com/integration",
		inner: "integration",
		outer: "",
	},
	Link{
		url: "https://basic-calculus.herokuapp.com/root-finding",
		inner: "root-finding",
		outer: "",
	},
	Link{
		url: "https://basic-calculus.herokuapp.com/max-finding",
		inner: "max-finding",
		outer: "",
	},
	Link{
		url: "https://basic-calculus.herokuapp.com/ode",
		inner: "1st order",
		outer: " differential equations",
	},
	Link{
		url: "https://basic-calculus.herokuapp.com/ode2",
		inner: "2nd order",
		outer: " differential equations",
	},
];

fn differentiation() -> LongPage {
	LongPage {
		title: "DIFFERENTIATION".to_string(),
		links: links(2),
		instructions: "In the url bar after <tt>https://basic-calculus.herokuapp.com/differentiation</tt> type the following:<p align=center><tt>&sol;&lt;value of <i>x</i> at which to calculate function and derivatives&gt;&sol;&lt;function of <i>x</I>&gt;</tt></p>".to_string(),
		note: format!("{}{}{}", NOTE1, "", NOTE2),
		example: "To differentiate the function 2<i>x</i> + 3/(<i>x</i><sup>4</sup> + 5) at <i>x</i> = 1, type <tt>/1/2x+3d(x**4+5)</tt> after the current url address. The results for the values of the function and of its first three derivatives should be <tt>2.5, 1.66..., -0.55..., and 1.11...</tt>".to_string(),
		algorithm: "finite differences for small values of &Delta;<i>x</i>, excluding any reference to the particular point itself in the case of a removable singularity".to_string(),
		json: "Type '/json' in the url bar immediately after 'differentiation' if you would like the result in this format rather than html.  A successful response will contain three properties: 'x' (a float), 'nonsingular' (a boolean reflecting whether or not the function has a removable singularity), and 'derivs' (a 4-element array of floats whose values represent the function value and first through third derivatives, respectively).  An unsuccessful response will have one property: 'message' (a string reporting the error).".to_string(),
	}
}

fn integration() -> LongPage {
	LongPage {
		title: "INTEGRATION".to_string(),
		links: links(3),
		instructions: "In the url bar after <tt>'https://basic-calculus.herokuapp.com/integration</tt> type the following:<p align=center>&sol;&lt;lower limit of integration&gt;&sol;&lt;upper limit of integration&gt;&sol;&lt;function of <i>x</I>&gt;</tt></p>Neither singularities (integrable or otherwise) nor infinite ranges of integration are allowed.".to_string(),
		note: format!("{}{}{}", NOTE1, "", NOTE2).to_string(),
		example: "To integrate the function 2<i>x</i> + 3/(<i>x</i><sup>4</sup> + 5) from <i>x</i> = 1 to 6, type <tt>/1/6/2x+3d(x**4+5)</tt> after the current url address.  The result for this should be <tt>35.41...</tt>".to_string(),
		algorithm: "composite Simpson's rule and Aitken extrapolation".to_string(),
		json: "Type '/json' in the url bar immediately after 'integration' if you would like the result in this format rather than html.  A successful response will contain five properties. 'xi' and 'xf' are the lower and upper limits of integration, 'integral' is the value of the definite integral, and 'subdivisions' is the number of equally sized intervals into which the range of integration needed to be subdivided in order to achieve the absolute accuracy specified in the last property: 'epsilon'. An unsuccessful response will have one property: 'message' (a string reporting the error)".to_string(),
	}
}

fn root_finding() -> LongPage {
	LongPage {
		title: "ROOT-FINDING".to_string(),
		links: links(4),
		instructions: "In the url bar after <tt>'https://basic-calculus.herokuapp.com/root-finding</tt> type the following:<p align=center>&sol;&lt;point at which to start search for a root&gt;&sol;&lt;function of <i>x</I>&gt;</tt></p>Note that this will not necessarily find the root which is <i>closest</i> to the input point.".to_string(),
		note: format!("{}{}", NOTE1, NOTE2).to_string(),
		example: "To find a root of the function 2<i>x</i> - 3/(<i>x</i><sup>4</sup> + 5) while starting the search at <i>x</i> = 1, type <tt>/1/2x-3d(x**4+5)</tt> after the current url address.  The result for this should be <tt>0.2995...</tt>".to_string(),
		algorithm: "alternating steps of inverse quadratic interpolation and simple bisection".to_string(),
		json: "Type '/json' in the url bar immediately after 'root-finding' if you would like the result in this format rather than html.  A successful response will contain five properties. 'xi' is the location where the search starts, 'x' is the root that is eventually found, 'bracket_steps' is the number of steps required to find numbers on either side of (ie, to 'bracket') the root, and 'root_steps' is the subsequent number of steps required for the algorithm to find this root to within the absolute accuracy specified in the last property: 'epsilon'. An unsuccessful response will have one property: 'message' (a string reporting the error).".to_string(),
	}
}

fn max_finding() -> LongPage {
	LongPage {
		title: "MAX-FINDING".to_string(),
		links: links(5),
		instructions: "In the url bar after <tt>'https://basic-calculus.herokuapp.com/max-finding</tt> type the following:<p align=center>&sol;&lt;point at which to start search for a maximum&gt;&sol;&lt;function of <i>x</I>&gt;</tt></p>Note that this will not necessarily find the local maximum which is <i>closest</i> to the input point.".to_string(),
		note: format!("{}{}", NOTE1, NOTE2).to_string(),
		example: "To find a local maximum of the function sin <i>x</i> + <i>x</i>/2 while starting the search at <i>x</i> = 1, type <tt>/1/sin(x)+xd2</tt> after the current url address.  The coordinates for this result should be <tt>(2.094..., 1.913...)</tt>.  If you want to find a local m<i>in</I>imum, simply multiply your function by -1.".to_string(),
		algorithm: "simple bisection and quadratic interpolation".to_string(),
		json: "Type '/json' in the url bar immediately after 'max-finding' if you would like the result in this format rather than html.  A successful response will contain six properties. 'xi' is the location where the search starts, 'x' is where the search ends, 'f' is the function value there, 'bracket_steps' is the number of steps required to find numbers on either side of (ie, to 'bracket') the maximum, and 'max_steps' is the subsequent number of steps required for the algorithm to find this maximum to within the absolute accuracy specified in the last property: 'epsilon'. An unsuccessful response will have one property: 'message' (a string reporting the error).".to_string(),
	}
}

fn ode() -> LongPage {
	LongPage {
		title: "1ST-ORDER DIFFERENTIAL EQUATIONS".to_string(),
		links: links(6),
		instructions: "This page solves a differential equation of the form <i>dx/dt</I> = function of <I>x</I> and <I>t</I>, with a specified 'initial condition', ie a value of <I>x</I> when the 'time' <i>t</i> = 0.  In the url bar after <tt>'https://basic-calculus.herokuapp.com/ode</tt> type the following:<p align=center>&sol;&lt;initial value of <i>x</I>&gt;&sol;&lt;final value of <i>t</I>&gt;&sol;&lt;number of time-steps&gt;&sol;&lt;function of <i>x</I> and <i>t</I>&gt;</tt></p>".to_string(),
		note: format!("{}{}", NOTE1, NOTE2).to_string(),
		example: "To solve the equation dx/dt = 2x - t - 2 from t = 0 to t = 2 using 10 time steps and the initial condition that x(0) = 1, type <tt>/1/2/10/2x-t-2</tt> after /ode in the url above.  The final result should be that x(2) = -11.39..".to_string(),
		algorithm: "4th-order Runge-Kutta method".to_string(),
		json: "Type '/json' in the url bar immediately after 'ode' if you would like the result in this format rather than html.  All of the data are returned.".to_string(),
	}
}

fn ode2() -> LongPage {
	LongPage {
		title: "2ND-ORDER DIFFERENTIAL EQUATIONS".to_string(),
		links: links(7),
		instructions: "This page solves a differential equation of the form <i>d</I><sup>2</sup><i>x/dt</i><sup>2</sup> = function of <I>x</I>, of <I>dx/dt</I> (= '<i>v</I>'), and of 'time' <I>t</I>, with a specified 'initial condition', ie values of <I>x</I> and of <i>v</I> when the 'time' <i>t</i> = 0. In the url bar after <tt>'https://basic-calculus.herokuapp.com/ode2</tt> type the following:<p align=center>&sol;&lt;initial value of <i>x</I>&gt;&sol;&lt;initial value of <i>v</I> v&gt;&sol;&lt;final value of <i>t</I>&gt;&sol;&lt;number of time-steps&gt;&sol;&lt;function of <i>x</I>, <i>v</I>, and <i>t</I>&gt;</tt></p>".to_string(),
		note: format!("{}{}", NOTE1, NOTE2).to_string(),
		example: "To solve the equation d<sup>2</sup>/dt<sup>2</sup> = -2x - v + 3t with the initial conditions that x(0) = 0 and dx/dt = v(0) = 1 over the range 0 < t < 4 using 10 time-steps, type <tt>/0/1/4/10/-2x-v+3t</tt> after /ode2 in the url above.  In this case the final values for x and dx/dt should be 5.31... and 1.57..., respectively.".to_string(),
		algorithm: "4th-order Runge-Kutta method".to_string(),
		json: "Type '/json' in the url bar immediately after 'ode2' if you would like the result in this format rather than html.  All data are returned.".to_string(),
	}
}

fn format(long_page: LongPage) -> String {
	format!("<p align=center>{}</p>{}<br>{} {}<br>{}<br><b>example:</b> {}<br><b>algorithms:</b> {}<br><b>json:</b> {}",
		long_page.title,
		long_page.links,
		long_page.instructions,
		FUNCTION,
		long_page.note,
		long_page.example,
		long_page.algorithm,
		long_page.json,
	)
}

pub fn general_page() -> String {format!("<p align=center>{}</p><p align=center>{}</p>", INSTRUCTIONS, links(1))}
pub fn differentiation_page() -> String {format(differentiation())}
pub fn integration_page() -> String {format(integration())}
pub fn root_finding_page() -> String {format(root_finding())}
pub fn max_finding_page() -> String {format(max_finding())}
pub fn ode_page() -> String {format(ode())}
pub fn ode2_page() -> String {format(ode2())}

fn links(n: i32) -> String {
	let mut links = "".to_string();
	for i in 0..8 {
		if i != n {
			links = format!("{}<a href='{}'>{}</a>{}<br>", links,
			  	LINKS[i as usize].url,
			  	LINKS[i as usize].inner,
			  	LINKS[i as usize].outer,
			);
		}
	}
	links
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DifferentiateResults {
	pub x: f64,
	pub nonsingular: bool,
	pub derivs: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IntegrateResults {
	pub xi: f64,
	pub xf: f64,
	pub integral: f64,
	pub subdivisions: i32,
	pub epsilon: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RootFindingResults {
	pub xi: f64,
	pub x: f64,
	pub bracket_steps: i32,
	pub root_steps: i32,
	pub epsilon: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaxFindingResults {
	pub xi: f64,
	pub x: f64,
	pub f: f64,
	pub bracket_steps: i32,
	pub max_steps: i32,
	pub epsilon: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ODEResults {
	pub xi: f64,
	pub tf: f64,
	pub nt: i32,
	pub xs: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ODE2Results {
	pub xi: f64,
	pub vi: f64,
	pub tf: f64,
	pub nt: i32,
	pub xs: Vec<f64>,
	pub vs: Vec<f64>,
}

pub fn differentiate_raw (x_str: &RawStr, input_str: &RawStr) -> Result<DifferentiateResults, String> {
	let x = match helper::parse_expression(x_str.to_string()) {
	  Ok(x) => x,
	  Err(message) => return Err(message),
	};
	let f = helper::function1(input_str.to_string(), x);
	let dx = 0.001;
	let steps = vec![2., 1., -1., -2.];
	let mut fs = vec![];
	for step in steps {
	  fs.push(match helper::function1(input_str.to_string(), x + step * dx) {
		Ok(f) => f,
		Err(message) => return Err(message),
	  });
	}
	let mut f0 = 0.;
	// I prob need to implement better testing for this.
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
	Ok(DifferentiateResults {
		x: x,
		nonsingular: nonsingular,
		derivs: derivs,
	})
}

pub fn integrate_raw(xi_str: &RawStr, xf_str: &RawStr, input_str: &RawStr) -> Result<IntegrateResults, String> {
	let epsilon = (10_f64).powf(-12.);
	struct Pt {
		x: f64,
		f: f64,
		wt: f64,
	}
	let mut pts = vec![];
	for x_str in &[xi_str, xf_str] {
		let x = match helper::parse_expression(x_str.to_string()) {
			Ok(x) => x,
			Err(message) => return Err(message),
		};
		let f = match helper::function1(input_str.to_string(), x) {
			Ok(f) => f,
			Err(message) => return Err(message),
		};
		pts.push(Pt{x, f, wt: 0.5}); // non-0th pt will only reside in vector for an instant
	}
	let ptf = match pts.pop() { // final point will be handled separately, going forward
	  	Some(ptf) => ptf,
	  	None => return Err("Missing integration endpoint".to_string()),
	};
	let mut integral = f64::INFINITY;
	// variables needed to implement Aitken's algo to accelerate a geometric sequence
	let mut aitkens = f64::INFINITY;
	let mut aitkens_new = f64::INFINITY;
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
			let f = match helper::function1(input_str.to_string(), x) {
			  	Ok(f) => f,
			  	Err(message) => return Err(format!("Cannot evaluate function at x: {}{}", pt.x, message)),
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
	Ok(IntegrateResults{
		integral: aitkens_new,
		xi: pts[0].x,
	  	xf: ptf.x,
		subdivisions: number,
		epsilon: epsilon,
	})
}

pub fn find_root_raw (xi_str: &RawStr, input_str: &RawStr) -> Result<RootFindingResults, String> {
	let epsilon = (10_f64).powf(-12.);
	let bracket_steps_max = 30;
	let xi = match helper::parse_expression(xi_str.to_string()) {
	  	Ok(x0) => x0,
	  	Err(message) => return Err(message),
	};
	// arbitrary
	let mut step = 0.1;
	// First, bracket the root.
	let mut x0 = xi - step / 2.;
	let mut x2 = xi + step / 2.;
	let mut f0 = match helper::function1(input_str.to_string(), x0) {
		Ok(f0) => f0,
		Err(message) => return Err(message),
	};
	let mut f2 = match helper::function1(input_str.to_string(), x2) {
		Ok(f2) => f2,
		Err(message) => return Err(message),
	};
	let mut bracket_steps = 0;
	while f0 * f2 > 0. {
		// golden mean is optimal for this
		step *= 1.6;
		if f0.abs() < f2.abs() {
			x0 -= step;
			f0 = match helper::function1(input_str.to_string(), x0) {
				Ok(f0) => f0,
				Err(message) => return Err(message),
			};
		} else {
			x2 += step;
			f2 = match helper::function1(input_str.to_string(), x2) {
				Ok(f2) => f2,
				Err(message) => return Err(message),
			};
		}
		bracket_steps += 1;
		if bracket_steps > bracket_steps_max {
			return Err(format!("Unable to bracket a root after {} steps.", bracket_steps_max));
		}
	}
	// Second, find a root that has been bracketed.
	let root_steps_max = 20;
	let mut root_steps = 0;
	// Utilize a third point, to allow inverse-quadratic interpolation.
	let mut x1 = (x0 + x2) / 2.;
	let mut f1 = match helper::function1(input_str.to_string(), x1) {
		Ok(f1) => f1,
		Err(message) => return Err(message),
	};
	let mut bisect = true;
	while f0.abs() > epsilon && f1.abs() > epsilon && f2.abs() > epsilon && (x2 - x1) * (x1 - x0) > epsilon * epsilon {
		bisect = !bisect;
		if root_steps > root_steps_max {
			return Err(format!("Unable to locate a bracketed root within {} steps.", root_steps_max));
		}
		// Alternate between bisection and inverse-quadratic interpolation to get the safety of the former and speed of the latter.
		if bisect {
			if f0 * f1 > 0. {
				let xc = (x1 + x2) / 2.;
				let fc = match helper::function1(input_str.to_string(), xc) {
					Ok(fc) => fc,
					Err(message) => return Err(message),
				};
				if fc * f2 > 0. {
					f2 = fc;
					x2 = xc;
				} else {
					f1 = fc;
					x1 = xc;
				}
			} else {
				let xc = (x1 + x0) / 2.;
				let fc = match helper::function1(input_str.to_string(), xc) {
					Ok(fc) => fc,
					Err(message) => return Err(message),
				};
				if fc * f0 > 0. {
					f0 = fc;
					x0 = xc;
				} else {
					f1 = fc;
					x1 = xc;
				}
			}
		} else {
			// inverse-quadratic interpolation (See wikipedia.)
			let xc = x0 * f1 * f2 / (f0 - f1) / (f0 - f2) +
			         x1 * f2 * f0 / (f1 - f0) / (f1 - f2) +
					 x2 * f0 * f1 / (f2 - f0) / (f2 - f1);
			// if interpolation results are outside brackets, skip to bisection iteration
			if xc < x0 || xc > x2 {
				continue;
			}
			let fc = match helper::function1(input_str.to_string(), xc) {
				Ok(fc) => fc,
				Err(message) => return Err(message),
			};
			if fc * f1 > 0. {
				if xc < x1 {
					x2 = x1;
					f2 = f1;
				} else {
					x0 = x1;
					f0 = f1;
				}
				x1 = xc;
				f1 = fc;
			} else {
				if fc * f0 > 0. {
					x0 = xc;
					f0 = fc;
				} else {
					x2 = xc;
					f2 = fc;
				}
			}
		}
		if f1 == 0. {
			break;
		}
		root_steps += 1;
	}
	if f0 * f1 <= 0. {
		x1 = if f0.abs() < f1.abs() {x0} else {x1};
	} else {
		x1 = if f2.abs() < f1.abs() {x2} else {x1};
	}
	Ok(RootFindingResults {
		xi,
		x: x1,
		bracket_steps,
		root_steps,
		epsilon,
	})
}

pub fn find_max_raw (xi_str: &RawStr, input_str: &RawStr) -> Result<MaxFindingResults, String> {
	let max_steps_max = 50;
	let epsilon = (10_f64).powf(-5.);
	let bracket_steps_max = 30;
	let xi = match helper::parse_expression(xi_str.to_string()) {
	  	Ok(xi) => xi,
	  	Err(message) => return Err(message),
	};
	let mut x1 = xi;
	// arbitrary
	let mut step = 0.1;
	// First, bracket the root.
	let mut x0 = x1 - step / 2.;
	let mut x2 = x1 + step / 2.;
	let mut f0 = match helper::function1(input_str.to_string(), x0) {
		Ok(f0) => f0,
		Err(message) => return Err(message),
	};
	let mut f1 = match helper::function1(input_str.to_string(), x1) {
		Ok(f1) => f1,
		Err(message) => return Err(message),
	};
	let mut f2 = match helper::function1(input_str.to_string(), x2) {
		Ok(f2) => f2,
		Err(message) => return Err(message),
	};
	let mut bracket_steps = 0;
	while f1 < f0 || f1 < f2 {
		// golden ratio
		step *= 1.6;
		if f2 > f0 {
			x0 = x1;
			f0 = f1;
			x1 = x2;
			f1 = f2;
			x2 += step;
			f2 = match helper::function1(input_str.to_string(), x2) {
				Ok(f2) => f2,
				Err(message) => return Err(message),
			};
		} else {
			x2 = x1;
			f2 = f1;
			x1 = x0;
			f1 = f0;
			x0 -= step;
			f0 = match helper::function1(input_str.to_string(), x0) {
				Ok(f0) => f0,
				Err(message) => return Err(message),
			};
		}
		bracket_steps += 1;
		if bracket_steps > bracket_steps_max {
			return Err(format!("Unable to bracket a max after {} steps.", bracket_steps_max));
		}
	}
	let mut max_steps = 0;
	// Following two vars will be sequential estimates using parabolic interpolation.
	let mut x_old = -f64::INFINITY;
	let mut x_new = f64::INFINITY;
	while (x_old - x_new).abs() > epsilon {
		if max_steps > max_steps_max {
			return Err(format!("Unable to locate a bracketed max within {} steps.", max_steps_max));
		}
		// Bisect the segment for which the outer function value is smallest.
		let x = (x1 + if f0 > f2 { x2 } else { x0 }) / 2.;
		let f = match helper::function1(input_str.to_string(), x) {
			Ok(f) => f,
			Err(message) => return Err(message),
		};
		if x < x1 {
			if f < f1 {
				x0 = x;
				f0 = f;
			} else {
				x2 = x1;
				f2 = f1;
				x1 = x;
				f1 = f;
			}
		} else {
			if f < f1 {
				x2 = x;
				f2 = f;
			} else {
				x0 = x1;
				f0 = f1;
				x1 = x;
				f1 = f;
			}
		}
		x_old = x_new;
		// parabolic interpolation
		let num = (x1 - x0) * (x1 - x0) * (f1 - f2) - (x1 - x2) * (x1 - x2) * (f1 - f0);
		let den = (x1 - x0) * (f1 - f2) - (x1 - x2) * (f1 - f0);
		x_new = x1 - num / den / 2.;
		max_steps += 1;
	}
	let f = match helper::function1(input_str.to_string(), x_new) {
		Ok(f) => f,
		Err(message) => return Err(message),
	};

	Ok(MaxFindingResults {
		xi,
		x: x_new,
		f,
		bracket_steps,
		max_steps,
		epsilon,
	})
}

pub fn ode_raw (xi_str: &RawStr, tf_str: &RawStr, nt_str: &RawStr, input_str: &RawStr) -> Result<ODEResults, String> {
	let xi = match helper::parse_expression(xi_str.to_string()) {
	  	Ok(x0) => x0,
	  	Err(message) => return Err(message),
	};
	let tf = match helper::parse_expression(tf_str.to_string()) {
		Ok(tf) => tf,
		Err(message) => return Err(message),
  	};
	let nt = match helper::parse_expression(nt_str.to_string()) {
		Ok(nt) => {
			if nt.round() != nt {
				return Err(format!("{} is not an integer.", nt));
			} else if nt <= 0. {
				return Err("number of timesteps must be positive.".to_string());
			}
			nt as i32
		},
		Err(message) => return Err(message),
  	};
	let mut xs = vec![xi];
	let dt = tf / (nt as f64);
	for i in 0..nt {
		let t = (i as f64) * tf / (nt as f64);
		let x = xs[i as usize];
		let v1 = match helper::function2(input_str.to_string(), x, t) {
			Ok(v) => v,
			Err(message) => return Err(message),
		};
		let v2 = match helper::function2(input_str.to_string(), x + v1 * dt / 2., t + dt / 2.) {
			Ok(v) => v,
			Err(message) => return Err(message),
		};
		let v3 = match helper::function2(input_str.to_string(), x + v2 * dt / 2., t + dt / 2.) {
			Ok(v) => v,
			Err(message) => return Err(message),
		};
		let v4 = match helper::function2(input_str.to_string(), x + v3 * dt, t + dt) {
			Ok(v) => v,
			Err(message) => return Err(message),
		};
		xs.push(x + ((v1 + v4) + 2. * (v2 + v3)) * dt / 6.);
	}
	return Ok(ODEResults {xi, tf, nt, xs});
}

pub fn ode2_raw (xi_str: &RawStr, vi_str: &RawStr, tf_str: &RawStr, nt_str: &RawStr, input_str: &RawStr) -> Result<ODE2Results, String> {
	let xi = match helper::parse_expression(xi_str.to_string()) {
	  	Ok(x0) => x0,
	  	Err(message) => return Err(message),
	};
	let vi = match helper::parse_expression(vi_str.to_string()) {
		Ok(v0) => v0,
		Err(message) => return Err(message),
  	};
	let tf = match helper::parse_expression(tf_str.to_string()) {
		Ok(tf) => tf,
		Err(message) => return Err(message),
  	};
	let nt = match helper::parse_expression(nt_str.to_string()) {
		Ok(nt) => {
			if nt.round() != nt {
				return Err(format!("{} is not an integer.", nt));
			} else if nt <= 0. {
				return Err("Number of timesteps must be positive.".to_string());
			}
			nt as i32
		},
		Err(message) => return Err(message),
  	};
	let mut xs = vec![xi];
	let mut vs = vec![vi];
	let dt = tf / (nt as f64);
	for i in 0..nt {
		let t = (i as f64) * tf / (nt as f64);
		let x = xs[i as usize];
		let v = vs[i as usize];
		let v1 = v;
		let a1 = match helper::function3(input_str.to_string(), x, t, v) {
			Ok(a) => a,
			Err(message) => return Err(message),
		};
		let v2 = v + a1 * dt / 2.;
		let a2 = match helper::function3(input_str.to_string(), x + v * dt / 2., t + dt / 2., v2) {
			Ok(a) => a,
			Err(message) => return Err(message),
		};
		let v3 = v + a2 * dt / 2.;
		let a3 = match helper::function3(input_str.to_string(), x + v2 * dt / 2., t + dt / 2., v3) {
			Ok(a) => a,
			Err(message) => return Err(message),
		};
		let v4 = v + a3 * dt;
		let a4 = match helper::function3(input_str.to_string(), x + v3 * dt, t + dt, v4) {
			Ok(a) => a,
			Err(message) => return Err(message),
		};
		xs.push(x + ((v1 + v4) + 2. * (v2 + v3)) * dt / 6.);
		vs.push(v + ((a1 + a4) + 2. * (a2 + a3)) * dt / 6.);
	}
	return Ok(ODE2Results {xi, vi, tf, nt, xs, vs});
}
