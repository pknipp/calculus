use std::f64::consts::PI;
use rocket::http::RawStr;
use serde::{Serialize, Deserialize};

mod helper;

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
pub struct ODE2Results {
	pub xi: f64,
	pub vi: f64,
	pub tf: f64,
	pub nt: i32,
	pub xs: Vec<f64>,
	pub vs: Vec<f64>,
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
