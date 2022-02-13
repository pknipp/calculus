use std::f64::consts::PI;

pub const INSTRUCTIONS: &str = "TITLE: Welcome to my app, which calculates a definite integral over a finite interval.\n\nINSTRUCTIONS: In the url bar after 'https://basic-calculus.herokuapp.com', type the following six items, in order:\n\n'/'\nlower limit of integration\n'/'\nupper limit of integration\n'/'\nany function of x\n\nThe function ('integrand') may be any algebraically legal combination of x, numbers, parentheses, and operations +, -, *, ** (or ^), PI and/or the most common unary functions: abs, acos, acosh, acot, acoth, acsc, acsch, asec, asech, asin, asinh, atan, atanh, cbrt, ceil, cos, cot, csc, exp, floor, ln, log10, log2, sec, sin, sqrt, tan, and trunc.  (See https://doc.rust-lang.org/std/primitive.f64.html for more information about any of these.) To represent division you must use either div, DIV, d, or D, because the usual division symbol ('/') has special meaning in a url.  Implied multiplication is allowed.  Spaces are allowed but discouraged. The rules for constructing either endpoint of integration are actually the same as those for the integrand except - of course - it cannot include x.  \n\nEXAMPLE: To integrate the function 2x+3/(x^4+5) from 1 to 6, type /1/6/2x+3d(x**4+5) after the current url address\nThe result for this should be 35.41358...\n\nALGORITHM: Composite Simpson's rule and Aitken extrapolation\n\nCOMING SOON: limits and derivative.\n\n";

// precedence of binary operations
fn prec(op: &char) -> i32 {
	match op {
		'+'|'-' => 0,
		'*'|'/' => 1,
		'^' => 2,
		_ => unreachable!(),
	}
}

pub fn function(x: f64, fn_str: &str) -> Result<f64, String> {
	let mut expression = fn_str.to_lowercase();
	// temporary swap-out of exp-spelling prevents confusion when inserting x value.
	expression = str::replace(&expression, "exp", &format!("{}", "EXP"));
	expression = str::replace(&expression, "x", &format!("({})", x));
	expression = str::replace(&expression, "EXP", &format!("{}", "exp"));
	parse_expression((expression).to_string())
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
	Err(format!("No closing parenthesis was found for this string: {}", expression))
}

fn get_value(expression: &mut String) -> Result<f64, String> {
	if expression.is_empty() {
		return Err("Your expression truncates prematurely.".to_string());
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
			return Err(format!("The unary function {} does not seem to have an argument.", method));
		}
		let n_expression = match find_size(expression) {
			Ok(n_expression) => n_expression,
			Err(message) => return Err(format!("Could not find length of argument string ({}) for function ({}): {}", expression, method, message)),
		};
		// recursive call, for argument of unary
		let arg = match parse_expression((expression)[..n_expression].to_string()) {
			Ok(arg) => arg,
			Err(message) => return Err(format!("Could not parse argument {}: {}", expression, message)),
		};
		value = match unary(&method, arg) {
			Ok(value) => value,
			Err(message) => return Err(message),
		};
		// Trim argument of unary from beginning of expression
		*expression = expression.split_off(n_expression + 1);
	} else {
		let mut p = 1; // index which tracks progress thru expression
		while expression.len() >= p {
			// If implied multiplication is detected ...
			if expression.chars().nth(p - 1).unwrap() == '(' {
				// ... insert a "*" symbol, to make things explicit, and restart
				expression.insert(p - 1, '*');
				break
			}
			let x = &expression[..p];
			if !(x == "." || x == "-" || x == "-.") {
				value = match x.parse() {
					Ok(value) => value,
					Err(_) => break, // we have found end of number, so value is finalized
				};
			}
			p += 1;
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
				return Err("attempted to divide by zero".to_string());
			}
			x1 / x2
		},
		'^' => {
			if x2 <= 0. && x1 == 0. {
				return Err(format!("{}^{} is ill-defined.", x1, x2));
			}
			x1.powf(x2)
		},
		_ => unreachable!(),
	};
	Ok(x)
}

pub fn parse_expression(mut expression: String) -> Result<f64, String> {
  	expression = str::replace(&expression.to_lowercase(), " ", ""); // simplify parsing
	expression = str::replace(&expression, "pi", &format!("({})", PI)); // important constant
  	for stri in ["d", "div", "DIV", "D"] {
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

// fn is_nonzero(x: f64) => fn ... used for fns w/poles
// fn is_positive(x: f64) => fn ... used for fns w/branch cuts: logs & sqrt (& inverse trigs?)

fn is_nonzero(x: f64) -> Result<f64, String> {
	if x == 0. {Err("divide by zero".to_string())} else {Ok(x)}
}

fn unary(method: &str, x: f64) -> Result<f64, String> {
	let negative = "is not defined for negative argument";
	let nonpositive = "is not defined for an argument which is not positive";
	match method {
		"abs" => Ok(x.abs()),
		"acos" => Ok(x.acos()),
		"acosh" => {
			if x >= 1. {
				Ok(x.acosh())
			} else {
				Err(format!("Error evaluating cosh({}): argument may not be smaller than 1.", x))
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
				return Err(format!("Error evaluating acoth({}): argment's absolute value must exceed 1.", x))
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
		"floor" => Ok(x.floor()),
		"ln" => {
			if x > 0. {
				Ok(x.ln())
			} else {
				Err(format!("ln {}", nonpositive))
			}
		},
		"log10" => {
			if x > 0. {
				Ok(x.log10())
			} else {
				Err(format!("log10 {}", nonpositive))
			}
		},
		"log2" => {
			if x > 0. {
				Ok(x.log2())
			} else {
				Err(format!("log2 {}", nonpositive))
			}
		},
		"sec" => Ok(1./x.cos()),
		"sin" => Ok(x.sin()),
		"sqrt" => {
			if x < 0. {
				Err(format!("sqrt {}", negative))
			} else {
				Ok(x.sqrt())
			}
		},
		"tan" => Ok(x.tan()),
		"trunc" => Ok(x.trunc()),
		_ => Err(format!("no such function: {}", method)),
	}
}
