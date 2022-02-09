fn precedence(op: &str) -> Result<i32, String> {
	let prec = match op {
		"+"|"-" => 0,
		"*"|"/" => 1,
		"^" => 2,
		_ => return Err("unknown operation".to_string()),
	};
	Ok(prec)
}

fn find_size (expression: &String) -> Result<usize, String> {
	println!("find_size starts with belief that expression = {}", expression);
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

fn get_value(expression: &mut String) -> Result<f64, String> {
	println!("get_value starts with belief that expression = {}", expression);
	if expression == "" {
		println!("l32 error");
		return Err("Your expression truncates prematurely.".to_string());
	}
	let mut value: f64 = 0.;
	if expression.chars().next().unwrap() == '(' {
		// remove leading parenthesis
		expression.remove(0);
		let n_expression = match find_size(&expression) {
			Ok(n_expression) => n_expression,
			Err(message) => return Err(message),
		};
		// recursive call to evalulate what is in parentheses
		value = match parse_expression((&expression[..n_expression]).to_string()) {
			Err(message) => return Err(message),
			Ok(value) => value,
		};
		// From expression remove trailing parenthesis and stuff preceding it.
		// Find a better way to do this.
		for _ in 0..n_expression + 1 {
			expression.remove(0);
		}
		return Ok(value);
	} else {
		println!("inside 2nd if block in get_value");
		// The following'll change only if strconv.ParseFloat ever returns no error, below.
		let mut p = 1;
		while expression.len() >= p {
			println!("in while loop, expression = {}", expression);
			// If implied multiplication is detected ...
			if &expression[p-1..p] == "(" {
				println!("inside 1st if block in while loop");
				// ... insert a "*" symbol.
				// expression = format!("{}*{}", expression[0..p-1], expression[p-1..]);
				expression.insert(p, '*');
				break
			}
			let x = &expression[..p];
			if !(x == "." || x == "-" || x == "-.") {
				println!("inside 2nd if block of while loop");
				value = match x.parse() {
					Ok(value) => value,
					Err(message) => {
						println!("in Err arm on l74: {}/{}", x, message.to_string());
						return Err(message.to_string());
					},
				};
			}
			p += 1;
		}
		for _ in 0..p - 1 {
			expression.remove(0);
		}
		// expression = expression[p-1..];
	}
	println!("get_expression ends successfully with belief that expression = {}", expression);
	Ok(value)
}

fn binary(x1: f64, op: &str, x2: f64) -> Result<f64, String> {
	let x = match op {
		"+" => x1 + x2,
		"-" => x1 - x2,
		"*" => x1 * x2,
		"/" => {
			if x2 == 0. {
				return Err("attempted to divide by zero".to_string());
			}
			x1 / x2
		},
		"^" => {
			if x2 <= 0. && x1 == 0. {
				return Err(format!("{}^{} is ill-defined.", x1, x2));
			}
			x1.powf(x2)
		},
		_ => return Err(format!("The operation {} is unknown.", op)),
	};
	Ok(x)
}

pub fn parse_expression(mut expression: String) -> Result<f64, String> {

	// struct fields consist of binary operation and 2nd number of the pair
	struct OpVal {
		op: String,
		val: f64,
	}
	println!("parse_expression starts with belief that expression = {}", expression);
	if expression != "" {
		// leading "+" may be trimmed thoughtlessly
		if expression.chars().next().unwrap() == '+' {
			expression.remove(0);
		}
	}
	let mut pairs = vec![];
	// trim&store leading number from expression
	let mut value = match crate::get_value(&mut expression) {
		Err(message) => return Err(message),
		Ok(value) => value,
	};
	let ops = "+-*/^";
	// loop thru the expression, while trimming off (and storing in "pairs" slice) operation/number pairs
	while expression != "" {
		let mut op = expression.chars().next().unwrap();
		if ops.contains(op) {
			expression.remove(0);
		} else {
			// It must be implied multiplication, so overwrite value of op.
			op = '*';
		}
		match get_value(&mut expression) {
			Err(message) => return Err(message),
			Ok(next_value) => {
				pairs.push(OpVal{op: op.to_string(), val: next_value});
			},
		}
	}
	// loop thru "pairs" slice, evaluating operations in order of their precedence
	while !pairs.is_empty() {
		let mut index = 0;
		while pairs.len() > index {
			let prec0 = match precedence(&pairs[index].op) {
				Ok(prec) => prec,
				Err(message) => return Err(message.to_string()),
			};
			let prec1 = match precedence(&pairs[index + 1].op) {
				Ok(prec) => prec,
				Err(message) => return Err(message.to_string()),
			};
			if index < pairs.len() - 1 && prec0 < prec1 {
				// postpone this operation because of its lower prececence
				index += 1;
			} else {
				// perform this operation NOW
				let f1:f64;
				//let result:f64;
				if index == 0 {
					f1 = value;
				} else {
					f1 = pairs[index-1].val;
				}
				match binary(f1, &pairs[index].op, pairs[index].val) {
					Err(message) => return Err(message),
					Ok(result) => {
						// mutate the values of value and pairs (reducing the length of the latter by one)
						if index == 0 {
							value = result;
							pairs.remove(0);
						} else {
							pairs[index-1].val = result;
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
