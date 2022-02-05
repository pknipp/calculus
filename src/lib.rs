// func handler(expression string) (string, [][2]string, [][2]string) {
	// expression = expression[1:] This was used when I used r.URL.path
	// result, message := parseExpression(expression)
	// posUnits := [][2]string{}
	// negUnits := [][2]string{}
	// if len(message) != 0 {
		// return "ERROR: " + message, posUnits, negUnits
	// }
	// realPart := strconv.FormatFloat(real(result.val), 'f', -1, 64)
	// imagPart := strconv.FormatFloat(math.Abs(imag(result.val)), 'f', -1, 64)
	// for _, unit := range result.units {
		// var powString string
		// reFloat, imFloat := int(real(unit.power)), int(imag(unit.power))
		// if float64(reFloat) == real(unit.power) {
			// if math.Abs(real(unit.power)) == 1. {
				// powString = ""
			// } else {
				// powString = strconv.Itoa(int(math.Abs(real(unit.power))))
			// }
		// } else {
			// powString = fmt.Sprintf("%.2f", real(unit.power))
		// }
		// if imag(unit.power) != 0. {
			// if float64(imFloat) == imag(unit.power) {
				// powString += "+" + strconv.Itoa(int(math.Abs(imag(unit.power)))) + "i"
			// } else {
				// powString += "+" + fmt.Sprintf("%.2f", math.Abs(imag(unit.power))) + "i"
			// }
		// }
		// if real(unit.power) > 0 {
			// posUnits = append(posUnits, [2]string{unit.name,  powString})
		// } else if real(unit.power) < 0 {
			// negUnits = append(negUnits, [2]string{unit.name,  powString})
		// }
	// }
	// var resultString string
	// if real(result.val) != 0 {
		// resultString = realPart
	// }
	// if real(result.val) != 0 && imag(result.val) != 0 {
		// sign := " + "
		// if imag(result.val) < 0 {
			// sign = " - "
		// }
		// resultString += sign
	// }
	// if imag(result.val) != 0 {
		// if real(result.val) == 0 && imag(result.val) < 0 {
			// resultString += " - "
		// }
		// if math.Abs(imag(result.val)) != 1. {
			// resultString += imagPart
		// }
		// resultString += "i"
	// }
	// if real(result.val) == 0 && imag(result.val) == 0 {
		// resultString = "0"
	// }
	// return resultString, posUnits, negUnits
// }
//
