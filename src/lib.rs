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
