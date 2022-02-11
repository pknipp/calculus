fn prec(op: &char) -> i32 {
	match op {
		'+'|'-' => 0,
		'*'|'/' => 1,
		'^' => 2,
		_ => unreachable!(),
	}
}

fn _function(x_str: String, fn_str: String) -> Result<f64, String> {
	let expression = str::replace(&fn_str, "x", &format!("({})", x_str));
	parse_expression(expression)
}

fn find_size (expression: &str) -> Result<usize, String> {
	let mut n_paren = 1; // leading (open)paren has been found, in calling function
	for n_expression in 0..expression.len() {
		let char = &expression[n_expression..n_expression + 1];
		n_paren += if char == "(" {1} else if char == ")" {-1} else {0};
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
	let mut value: f64 = 0.;
	if expression.starts_with('(') {
		// remove leading parenthesis
		expression.remove(0);
		let n_expression = match find_size(expression) {
			Ok(n_expression) => n_expression,
			Err(message) => return Err(message),
		};
		// recursive call to evalulate what is in parentheses
		value = match parse_expression((&expression[..n_expression]).to_string()) {
			Err(message) => return Err(message),
			Ok(value) => value,
		};
		// From expression remove trailing parenthesis and stuff preceding it.
		*expression = expression.split_off(n_expression + 1);
		return Ok(value);
	} else {
		let mut p = 1;
		while expression.len() >= p {
			// If implied multiplication is detected ...
			if &expression[p-1..p] == "(" {
				// ... insert a "*" symbol.
				expression.insert(p - 1, '*');
				break
			}
			let x = &expression[..p];
			if !(x == "." || x == "-" || x == "-.") {
				value = match x.parse() {
					Ok(value) => value,
					Err(_message) => break,
				};
			}
			p += 1;
		}
		*expression = expression.split_off(p - 1);
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

	if !expression.is_empty() {
		// leading "+" may be trimmed thoughtlessly
		if expression.starts_with('+') {
			expression.remove(0);
		}
	}
	// These two vectors are interleaved: val/op/val/op.../op/val
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
				// perform this operation NOW
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
	Ok(vals[0])
}
