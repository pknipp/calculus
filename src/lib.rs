fn precedence(op: &str) -> i32 {
	match op {
		"+"|"-" => 0,
		"*"|"/" => 1,
		"^" => 2,
		_ => return,
	}
}

fn get_value(expression: &str) -> Result<f64, String> {
	if expression == "" {
		return Err("Your expression truncates prematurely.".to_string());
	}
	let leadingChar = (*expression)[0:1];
	if leadingChar == "(" {
		// remove leading parenthesis
		expression = expression[1:];
		let n_expression = match findSize(expression) {
			Err(message) => return Err(message),
			Ok(n_expression) => n_expression,
		};
		// recursive call to evalulate what is in parentheses
		let value = match parseExpression((expression)[0:nExpression]) {
			Err(message) => return Err(message),
			Ok(value) => value,
		};
		// From expression remove trailing parenthesis and stuff preceding it.
		expression = (expression)[nExpression+1:]
		return Ok(value);
	} else {
		// The following'll change only if strconv.ParseFloat ever returns no error, below.
		message = "The string '" + expression + "' does not evaluate to a number."
		p := 1
		for len(*expression) >= p {
			// If implied multiplication is detected ...
			if z := (*expression)[0:p]; (*expression)[p-1:p] == "(" {
				// ... insert a "*" symbol.
				*expression = (*expression)[0:p-1] + "*" + (*expression)[p-1:]
				break
			} else if !(z == "." || z == "-" || z == "-.") {
				if num, err := strconv.ParseFloat(z, 64); err != nil {
					break
				} else {
					quantity.val = complex(num, 0.)
					message = ""
				}
			}
			p += 1;
		}
		*expression = (*expression)[p-1:];
		return Ok(value);
	}
	return Err(format!("Could not parse {} {}", leadingChar, expression));
}

fn parse_expression(mut expression: &str) -> Result<f64, String> {

	// struct fields consist of binary operation and 2nd number of the pair
	struct OpVal {
		op: String,
		val: f64,
	}

	if expression != "" {
		// leading "+" may be trimmed thoughtlessly
		if expression[0:1] == "+" {
			expression = expression[1:];
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
		let op = expression[0:1];
		if strings.Contains(OPS, op) {
			expression = expression[1:];
		} else {
			// It must be implied multiplication, so overwrite value of op.
			op = "*";
		}
		match get_value(expression) {
			Err(message) => return Err(message),
			Ok(next_value) => {
				pairs.push(op_quant{op, next_value});
			},
		}
	}
	// loop thru "pairs" slice, evaluating operations in order of their precedence
	while !pairs.is_empty() {
		let index = 0;
		while pairs.len() > index {
			if index < pairs.len() - 1 && PRECEDENCE[pairs[index].op] < PRECEDENCE[pairs[index+1].op] {
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
				match binary(q1, pairs[index].op, pairs[index].value) {
					Err(message) => return Err(message),
					Ok(result) => {
						// mutate the values of z and pairs (reducing the length of the latter by one)
						if index == 0 {
							value = result;
							pairs = pairs[1:];
						} else {
							pairs[index-1].value = result;
							pairs = append(pairs[0:index], pairs[index+1:]...);
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
