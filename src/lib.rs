use std::f64::consts::PI;
use rocket::http::RawStr;
use serde::{Serialize, Deserialize};

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

const LINKS: [Link; 7] = [
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
		json: "Type '/json' in the url bar immediately after 'root-finding' if you would like the result in this format rather than html.  A successful response will contain five properties. 'xi' is the location where the search starts, 'x' is the root that is eventually found, 'bracket_steps' is the number of steps required to find numbers on either side of (ie, to 'bracket') the root, and 'root_steps' is the subsequent number required for the algorithm to find this root to within the absolute accuracy specified in the last property: 'epsilon'. An unsuccessful response will have one property: 'message' (a string reporting the error).".to_string(),
	}
}

fn ode() -> LongPage {
	LongPage {
		title: "1ST-ORDER DIFFERENTIAL EQUATIONS".to_string(),
		links: links(5),
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
		links: links(6),
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
pub fn ode_page() -> String {format(ode())}
pub fn ode2_page() -> String {format(ode2())}

fn links(n: i32) -> String {
	let mut links = "".to_string();
	for i in 0..7 {
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

// precedence of binary operations
fn prec(op: &char) -> i32 {
	match op {
		'+'|'-' => 0,
		'*'|'/' => 1,
		'^' => 2,
		_ => unreachable!(),
	}
}

// pub fn preparse (fn_str: &str) -> String {
	// let mut expression = fn_str.to_lowercase();
	// following are replacements of url encoding of ^ and space, respectively.
	// expression = str::replace(&expression, "%5e", &"^".to_string());
	// expression = str::replace(&expression, "%20e", &"".to_string());
	// temporary swap-out of exp-spelling prevents confusion when inserting x value.
	// str::replace(&expression, "exp", &"EXP".to_string())
	// expression = str::replace(&expression, "x", &format!("({})", x));
	// expression = str::replace(&expression, "EXP", &"exp".to_string())
// }

pub fn function1(expression: &str, x: f64) -> Result<f64, String> {
	//let mut expression = preparse(fn_str);
	let mut expression = expression.to_lowercase();
	expression = str::replace(&expression, "%5e", &"^".to_string());
	expression = str::replace(&expression, "%20", &"".to_string()); // %20 = url encoding of space
	// temporary swap-out of exp-spelling prevents confusion when inserting x value.
	expression = str::replace(&expression, "exp", &"EXP".to_string());
	expression = str::replace(&expression, "x", &format!("({})", x).to_string());
	expression = str::replace(&expression, "EXP", &"exp".to_string());
	parse_expression(expression.to_string())
}

pub fn function2(expression: &str, x: f64, t: f64) -> Result<f64, String> {
	//let mut expression = preparse(fn_str);
	let mut expression = expression.to_lowercase();
	expression = str::replace(&expression, "%5e", &"^".to_string());
	expression = str::replace(&expression, "%20", &"".to_string()); // %20 = url encoding of space
	// temporary swap-out of exp-spelling prevents confusion when inserting x value.
	expression = str::replace(&expression, "exp", &"EXP".to_string());
	expression = str::replace(&expression, "x", &format!("({})", x).to_string());
	expression = str::replace(&expression, "EXP", &"exp".to_string());
	expression = str::replace(&expression, "t", &format!("({})", t).to_string());
	parse_expression(expression.to_string())
}

pub fn function3(expression: &str, x: f64, t: f64, v: f64) -> Result<f64, String> {
	//let mut expression = preparse(fn_str);
	let mut expression = expression.to_lowercase();
	expression = str::replace(&expression, "%5e", &"^".to_string());
	expression = str::replace(&expression, "%20", &"".to_string()); // %20 = url encoding of space
	// temporary swap-out of exp-spelling prevents confusion when inserting x value.
	expression = str::replace(&expression, "exp", &"EXP".to_string());
	expression = str::replace(&expression, "x", &format!("({})", x).to_string());
	expression = str::replace(&expression, "EXP", &"exp".to_string());
	expression = str::replace(&expression, "t", &format!("({})", t).to_string());
	expression = str::replace(&expression, "v", &format!("({})", v).to_string());
	parse_expression(expression.to_string())
}

fn find_size (expression: &str) -> Result<usize, String> {
	let mut n_paren = 1; // leading (open)paren has been found, in calling function
	for (n_expression, char) in expression.chars().enumerate() {
		n_paren += if char == '(' {1} else if char == ')' {-1} else {0};
		if n_paren == 0 {
			// Closing parenthesis has been found.
			return Ok(n_expression);
		}
	}
	Err(format!("Error: no closing parenthesis was found for this string: {}", expression))
}

fn get_value(expression: &mut String) -> Result<f64, String> {
	if expression.is_empty() {
		return Err("Error: your expression truncates prematurely.".to_string());
	}
	let mut value = 0.;
	if expression.starts_with('(') {
		// remove leading parenthesis
		expression.remove(0);
		let n_expression = match find_size(expression) {
			Ok(n_expression) => n_expression,
			Err(message) => return Err(message),
		};
		// recursive call to evaluate what is in parentheses
		value = match parse_expression((&expression[..n_expression]).to_string()) {
			Err(message) => return Err(message),
			Ok(value) => value,
		};
		// From expression remove trailing parenthesis and characters preceding it.
		*expression = expression.split_off(n_expression + 1);
	// A letter here triggers that we are starting a unary function name (or E-notation?)
	} else if expression.chars().next().unwrap().is_alphabetic() {
		let mut method = String::from("");
		let mut found_paren = false;
		while !expression.is_empty() {
			let char = expression.remove(0);
			if char == '(' {
				found_paren = true;
				break;
			} else {
				method += &String::from(char);
			}
		}
		if !found_paren {
			return Err(format!("Error: the unary function {} does not seem to have an argument.", method));
		}
		let n_expression = match find_size(expression) {
			Ok(n_expression) => n_expression,
			Err(message) => return Err(format!("Error: could not find length of argument string ({}) for function ({}): {}", expression, method, message)),
		};
		// recursive call, for argument of unary
		let arg = match parse_expression((expression)[..n_expression].to_string()) {
			Ok(arg) => arg,
			Err(message) => return Err(format!("Error: could not parse argument {}: {}", expression, message)),
		};
		value = match unary(&method, arg) {
			Ok(value) => value,
			Err(message) => return Err(message),
		};
		// Trim argument of unary from beginning of expression
		*expression = expression.split_off(n_expression + 1);
	} else {
		let mut found_value = false;
		let mut p = 1; // index which tracks progress thru expression
		let mut x = "";
		while expression.len() >= p {
			x = &expression[..p];
			if !(x == "." || x == "-" || x == "-.") { // It's premature to parse for a number.
				value = match x.parse() {
					Ok(value) => {
						found_value = true;
						value // This may get more digit(s) in next iteration(s).
					},
					Err(_) => break, // either found end of value or expression is unparsable
				};
			}
			p += 1;
		}
		if x.starts_with("-") && p == 2 && expression.len() > 1 { // examples of this edge case: -sin(x) or -(x+1)**2
			value = -1.;
			found_value = true;
		}
		if !found_value {
			return Err(format!("Error: cannot parse a number from the start of '{}'", expression));
		}
		*expression = expression.split_off(p - 1); //start of expression is no longer needed
	}
	Ok(value)
}

fn binary(x1: f64, op: &char, x2: f64) -> Result<f64, String> {
	let x = match op {
		'+' => x1 + x2,
		'-' => x1 - x2,
		'*' => x1 * x2,
		'/' => {
			if x2 == 0. {
				return Err(format!("Error: {}/0 signifies an attempt to divide by zero", x1));
			}
			x1 / x2
		},
		'^' => {
			if x2 <= 0. && x1 == 0. {
				return Err(format!("Error: {}^0 is ill-defined.", x1));
			}
			x1.powf(x2)
		},
		_ => unreachable!(),
	};
	Ok(x)
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
	let x = match parse_expression(x_str.to_string()) {
	  Ok(x) => x,
	  Err(message) => return Err(message),
	};
	// let expression = calculus::preparse(input_str);
	let f = function1(&input_str, x);
	let dx = 0.001;
	let steps = vec![2., 1., -1., -2.];
	let mut fs = vec![];
	for step in steps {
	  fs.push(match function1(&input_str, x + step * dx) {
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
		let x = match parse_expression(x_str.to_string()) {
			Ok(x) => x,
			Err(message) => return Err(message),
		};
		let f = match function1(input_str, x) {
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
			let f = match function1(input_str, x) {
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
	let xi = match parse_expression(xi_str.to_string()) {
	  	Ok(x0) => x0,
	  	Err(message) => return Err(message),
	};
	// arbitrary
	let mut step = 0.1;
	// First, bracket the root.
	let mut x0 = xi - step / 2.;
	let mut x2 = xi + step / 2.;
	let mut f0 = match function1(input_str, x0) {
		Ok(f0) => f0,
		Err(message) => return Err(message),
	};
	let mut f2 = match function1(input_str, x2) {
		Ok(f2) => f2,
		Err(message) => return Err(message),
	};
	let mut bracket_steps = 0;
	while f0 * f2 > 0. {
		// golden mean is optimal for this
		step *= 1.6;
		if f0.abs() < f2.abs() {
			x0 -= step;
			f0 = match function1(input_str, x0) {
				Ok(f0) => f0,
				Err(message) => return Err(message),
			};
		} else {
			x2 += step;
			f2 = match function1(input_str, x2) {
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
	let mut f1 = match function1(input_str, x1) {
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
				let fc = match function1(input_str, xc) {
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
				let fc = match function1(input_str, xc) {
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
			let fc = match function1(input_str, xc) {
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

pub fn ode_raw (xi_str: &RawStr, tf_str: &RawStr, nt_str: &RawStr, input_str: &RawStr) -> Result<ODEResults, String> {
	let xi = match parse_expression(xi_str.to_string()) {
	  	Ok(x0) => x0,
	  	Err(message) => return Err(message),
	};
	let tf = match parse_expression(tf_str.to_string()) {
		Ok(tf) => tf,
		Err(message) => return Err(message),
  	};
	let nt = match parse_expression(nt_str.to_string()) {
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
		let v1 = match function2(&input_str, x, t) {
			Ok(v) => v,
			Err(message) => return Err(message),
		};
		let v2 = match function2(&input_str, x + v1 * dt / 2., t + dt / 2.) {
			Ok(v) => v,
			Err(message) => return Err(message),
		};
		let v3 = match function2(&input_str, x + v2 * dt / 2., t + dt / 2.) {
			Ok(v) => v,
			Err(message) => return Err(message),
		};
		let v4 = match function2(&input_str, x + v3 * dt, t + dt) {
			Ok(v) => v,
			Err(message) => return Err(message),
		};
		xs.push(x + ((v1 + v4) + 2. * (v2 + v3)) * dt / 6.);
	}
	return Ok(ODEResults {xi, tf, nt, xs});
}

pub fn ode2_raw (xi_str: &RawStr, vi_str: &RawStr, tf_str: &RawStr, nt_str: &RawStr, input_str: &RawStr) -> Result<ODE2Results, String> {
	let xi = match parse_expression(xi_str.to_string()) {
	  	Ok(x0) => x0,
	  	Err(message) => return Err(message),
	};
	let vi = match parse_expression(vi_str.to_string()) {
		Ok(v0) => v0,
		Err(message) => return Err(message),
  	};
	let tf = match parse_expression(tf_str.to_string()) {
		Ok(tf) => tf,
		Err(message) => return Err(message),
  	};
	let nt = match parse_expression(nt_str.to_string()) {
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
		let a1 = match function3(&input_str, x, t, v) {
			Ok(a) => a,
			Err(message) => return Err(message),
		};
		let v2 = v + a1 * dt / 2.;
		let a2 = match function3(&input_str, x + v * dt / 2., t + dt / 2., v2) {
			Ok(a) => a,
			Err(message) => return Err(message),
		};
		let v3 = v + a2 * dt / 2.;
		let a3 = match function3(&input_str, x + v2 * dt / 2., t + dt / 2., v3) {
			Ok(a) => a,
			Err(message) => return Err(message),
		};
		let v4 = v + a3 * dt;
		let a4 = match function3(&input_str, x + v3 * dt, t + dt, v4) {
			Ok(a) => a,
			Err(message) => return Err(message),
		};
		xs.push(x + ((v1 + v4) + 2. * (v2 + v3)) * dt / 6.);
		vs.push(v + ((a1 + a4) + 2. * (a2 + a3)) * dt / 6.);
	}
	return Ok(ODE2Results {xi, vi, tf, nt, xs, vs});
}

pub fn parse_expression(mut expression: String) -> Result<f64, String> {
  	expression = str::replace(&expression.to_lowercase(), " ", ""); // simplify parsing
	expression = str::replace(&expression, "pi", &format!("({})", PI)); // important constant
  	for stri in ["div", "DIV", "d", "D"] {
    	expression = str::replace(&expression, stri, "/"); // division operation is a special URL char
  	}
	expression = str::replace(&expression, "**", "^"); // in case user chooses ^ instead of **

	if !expression.is_empty() {
		// leading "+" may be trimmed thoughtlessly
		if expression.starts_with('+') {
			expression.remove(0);
		}
	}
	// Elements of these two vectors are interleaved: val/op/val/op.../op/val
	let mut vals = vec![];
	let mut ops = vec![];
	// trim & push leading number from expression
	vals.push(match crate::get_value(&mut expression) {
		Err(message) => return Err(message),
		Ok(value) => value,
	});
	let op_string = "+-*/^";
	// loop thru the expression, while trimming & pushing operation/number pairs
	while !expression.is_empty() {
		let op = expression.chars().next().unwrap();
		// The following ternary includes an implied multiplication, if appropriate.
		ops.push(if op_string.contains(op) {expression.remove(0)} else {'*'});
		vals.push(match get_value(&mut expression) {
			Err(message) => return Err(message),
			Ok(value) => value,
		});
	}
	// loop thru "ops" vector, evaluating operations in order of their precedence
	while !ops.is_empty() {
		let mut index = 0;
		while ops.len() > index {
			if index < ops.len() - 1 && prec(&ops[index]) < prec(&ops[index + 1]) {
				// postpone this operation because of its lower prececence
				index += 1;
			} else {
				// perform this operation NOW, because of PEMDAS rule
				match binary(vals[index], &ops[index], vals[index + 1]) {
					Err(message) => return Err(message),
					Ok(result) => {
						// mutate vals & ops (including the shortening of both by one)
						vals[index] = result;
						ops.remove(index);
						vals.remove(index + 1);
					},
				};
				// Start another loop thru the expression, ISO high-precedence operations.
				index = 0;
			}
		}
	}
	Ok(vals[0]) // what remains after ops vector is emptied
}

fn is_nonzero(x: f64) -> Result<f64, String> {
	if x == 0. {Err("Error: divide by zero".to_string())} else {Ok(x)}
}

fn unary(method: &str, x: f64) -> Result<f64, String> {
	let negative = format!("is not defined for negative argument such as {}", x);
	let nonpositive = format!("is not defined for a nonpositive argument such as {}", x);
	match method {
		"abs" => Ok(x.abs()),
		"acos" => {
			if x<= 1. {
				Ok(x.acos())
			} else {
				return Err(format!("Error evaluating acos({}): your argument cannot exceed 1 in absolute value.", x))
			}
		},
		"acosh" => {
			if x >= 1. {
				Ok(x.acosh())
			} else {
				return Err(format!("Error evaluating acosh({}): your argument cannot be smaller than 1.", x))
			}
		},
		"acot" => {
			if x == 0. {
				Ok(PI/2.)
			} else if x > 0. {
				Ok((1./x).atan())
			} else {
				Ok(PI + (1./x).atan())
			}
		},
		"acoth" => {
			if x.abs() > 1. {
				Ok((1./x).atanh())
			} else {
				return Err(format!("Error evaluating acoth({}): argument's absolute value must exceed 1.", x))
			}
		},
		"acsc" => {
			if x.abs() >= 1. {
				Ok((1./x).asin())
			} else {
				return Err(format!("Error evaluating acsc({}): argument's absolute value may not be smaller than 1.", x))
			}
		},
		"acsch" => match is_nonzero(x) {
			Ok(x) => Ok((1./x).asinh()),
			Err(message) => return Err(format!("Error evaluating {}({}): {}", method, x, message)),
		},
		"asec" => {
			if x.abs() < 1. {
				return Err(format!("Error evaluating asec({}): argument's absoluate value may not be less than 1.", x))
			} else {
				Ok((1./x).acos())
			}
		},
		"asech" => {
			if x > 0. && x <= 1. {
				Ok((1./x).acosh())
			} else {
				return Err(format!("Error evaluating asech({}): argument must be between 0 (exclusive) and 1 (inclusive).", x))
			}
		},
		"asin" => {
			if x.abs() < 1. {
				return Err(format!("Error evaluating asin({}): argument's absolute value may not exceed 1.", x))
			} else {
				Ok(x.asin())
			}
		},
		"asinh" => Ok(x.asinh()),
		"atan" => Ok(x.atan()),
		"atanh" => {
			if x.abs() < 1. {
				Ok(x.atanh())
			} else {
				return Err(format!("Error evaluating atanh({}): argument's absolute value must be less than 1.", x))
			}
		},
		"cbrt" => Ok(x.cbrt()),
		"ceil" => Ok(x.ceil()),
		"cos" => Ok(x.cos()),
		"cot" => match is_nonzero(x) {
			Ok(x) => Ok(x.cos()/x.sin()),
			Err(message) => return Err(format!("Error evaluating {}({}): {}", method, x, message)),
		},
		"csc" => match is_nonzero(x) {
			Ok(x) => Ok(1./x.sin()),
			Err(message) => return Err(format!("Error evaluating {}({}): {}", method, x, message)),
		},
		"exp" => Ok(x.exp()),
		"exp2" => Ok(x.exp2()),
		"exp_m1" => Ok(x.exp_m1()),
		"floor" => Ok(x.floor()),
		"fract" => Ok(x.fract()),
		"ln" => {
			if x > 0. {
				Ok(x.ln())
			} else {
				Err(format!("Error: ln {}", nonpositive))
			}
		},
		"ln_1p" => {
			if x > -1. {
				Ok(x.ln_1p())
			} else {
				Err(format!("Error evaluating ln_1p({}): argument must exceed -1", x))
			}
		},
		"log10" => {
			if x > 0. {
				Ok(x.log10())
			} else {
				Err(format!("Error: log10 {}", nonpositive))
			}
		},
		"log2" => {
			if x > 0. {
				Ok(x.log2())
			} else {
				Err(format!("Error: log2 {}", nonpositive))
			}
		},
		"round" => Ok(x.round()),
		"sec" => Ok(1./x.cos()),
		"signum" => Ok(x.signum()),
		"sin" => Ok(x.sin()),
		"sqrt" => {
			if x < 0. {
				Err(format!("Error: sqrt {}", negative))
			} else {
				Ok(x.sqrt())
			}
		},
		"tan" => Ok(x.tan()),
		"trunc" => Ok(x.trunc()),
		_ => Err(format!("Error: no such function: {}", method)),
	}
}
