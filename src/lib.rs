fn precedence(op: &str) -> Result<i32, String> {
	let prec = match op {
		"+"|"-" => 0,
		"*"|"/" => 1,
		"^" => 2,
		_ => return Err("unknown operation".to_string()),
	};
	Ok(prec)
}

fn find_size (expression: String) -> Result<usize, String> {
	let mut n_paren = 1; // leading (open)paren has been found, in calling function
	for n_expression in 0..expression.len() {
		let char = &expression[n_expression..n_expression + 1];
		if char == "(" {
			n_paren += 1;
		} else if char == ")" {
			n_paren -= 1;
		}
		if n_paren == 0 {
			// Closing parenthesis has been found.
			return Ok(n_expression);
		}
	}
	Err(format!("No closing parenthesis was found for this string: {}", expression))
}

fn get_value(expression: String) -> Result<f64, String> {
	if expression == "" {
		return Err("Your expression truncates prematurely.".to_string());
	}
	let leadingChar = expression.chars().next().unwrap();
	let value: f64;
	if leadingChar == '(' {
		// remove leading parenthesis
		expression.remove(0);
		let n_expression = match find_size(expression) {
			Err(message) => return Err(message),
			Ok(n_expression) => n_expression,
		};
		// recursive call to evalulate what is in parentheses
		value = match parse_expression(&expression[..n_expression]) {
			Err(message) => return Err(message),
			Ok(value) => value,
		};
		// From expression remove trailing parenthesis and stuff preceding it.
		expression = expression[n_expression+1..].to_string();
		return Ok(value);
	} else {
		// The following'll change only if strconv.ParseFloat ever returns no error, below.
		let message = format!("The string \"{}\" does not evaluate to a number.", expression);
		let mut p = 1;
		while expression.len() >= p {
			// If implied multiplication is detected ...
			if &expression[p-1..p] == "(" {
				// ... insert a "*" symbol.
				expression = format!("{}*{}", expression[0..p-1], expression[p-1..]);
				break
			}
			let z = expression[..p];
			if !(z == "." || z == "-" || z == "-.") {
				let num: f64 = match z.parse() {
					Ok(num) => num,
					Err(message) => return Err(message),
				};
			}
			p += 1;
		}
		expression = expression[p-1..];
	}
	Ok(value)
}

fn binary(x1: f64, op: String, x2: f64) -> Result<f64, String> {
	let x = match op {
		"+" => x1 + x2,
		"-" => x1 - x2,
		"*" => x1 * x2,
		"/" => {
			if x2 == 0 {
				return Err("attempted to divide by zero");
			}
			x1 / x2;
		},
		"^" => {
			if x2 <= 0 && x1 == 0. {
				return Err("{}^{} is ill-defined.", x1, x2);
			}
			x1.powf(x2);
		},
		_ => return Err("The operation {} is unknown.", op),
	};
	Ok(x)
}

fn parse_expression(mut expression: &str) -> Result<f64, String> {

	// struct fields consist of binary operation and 2nd number of the pair
	struct OpVal {
		op: String,
		val: f64,
	}

	if expression != "" {
		// leading "+" may be trimmed thoughtlessly
		if expression.chars().next().unwrap() == "+" {
			expression.remove(0);
		}
	}
	let pairs = vec![];
	// trim&store leading number from expression
	let value = match crate::get_value(expression) {
		Err(message) => return Err(message),
		Ok(value) => value,
	};
	let OPS = "+-*/^";
	// loop thru the expression, while trimming off (and storing in "pairs" slice) operation/number pairs
	while expression != "" {
		let op = expression.chars().next().unwrap();
		if OPS.Contains(op) {
			expression.remove(0);
		} else {
			// It must be implied multiplication, so overwrite value of op.
			op = "*";
		}
		match get_value(expression) {
			Err(message) => return Err(message),
			Ok(next_value) => {
				pairs.push(OpVal{op, val: next_value});
			},
		}
	}
	// loop thru "pairs" slice, evaluating operations in order of their precedence
	while !pairs.is_empty() {
		let index = 0;
		while pairs.len() > index {
			if index < pairs.len() - 1 && precedence([pairs[index].op]) < precedence([pairs[index+1].op]) {
				// postpone this operation because of its lower prececence
				index += 1;
			} else {
				// perform this operation NOW
				let f1:f64;
				let result:f64;
				if index == 0 {
					f1 = value;
				} else {
					f1 = pairs[index-1].value;
				}
				match binary(f1, pairs[index].op, pairs[index].value) {
					Err(message) => return Err(message),
					Ok(result) => {
						// mutate the values of value and pairs (reducing the length of the latter by one)
						if index == 0 {
							value = result;
							pairs.remove(0);
						} else {
							pairs[index-1].value = result;
							pairs.remove(index);
						}
					},
				};
				// Start another loop thru the expression, ISO high-precedence operations.
				index = 0;
			}
		}
	}
	Ok(value)
}
