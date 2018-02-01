const _static_mapTokens = (e) => {
	let str = ""
	let sign = (e.factor >= 0) ? '+ ' : '- '

	e.factor = (e.factor < 0) ? -e.factor : e.factor;

	if (e.factor != 1 && e.factor != 0)
		str = str.concat(`${sign}${e.factor}`)
	else if (e.factor == 0)
		str = str.concat(`0`)
	else
		str = str.concat(`${sign}`)
	if (e.factor && e.order != 1 && e.order != 0)
		str = str.concat(`x ^ ${e.order}`)
	else if (e.factor && e.order == 1)
		str = str.concat(`x`)
	return str
}

function displayReduced(equation) {
	let strArray 

	strArray = equation.map(_static_mapTokens)
	strArray.push("= 0")
	console.log("Reduced form :", strArray.join(' '))
}

function displayDegree(degree) {
	console.log(`Polynomial degree : ${degree}`)
}

function displayDelta(delta) {
	console.log(`Discriminant: ${delta}`)
	if (delta > 0)
		console.log("Discriminant is positive : two solutions")
	else if (delta == 0)
		console.log("Discriminant is null : only one solution")
	else
		console.log("Discriminant is negative : two conjugate complex solutions")
}

function displaySolutions(solutions) {
	solutions.map((e, i) => {
		if (e.b) {
			let sign = (e.b >= 0) ? '+' : '-' 
			e.b = (e.b < 0) ? -e.b : e.b
			console.log(`x${i} =\t${e.a} ${sign} ${e.b}i`)
		} else {
			console.log(`x${i} =\t${e.a}`)
		}
	})
}

module.exports = {
	reduced: displayReduced,
	delta: displayDelta,
	degree: displayDegree,
	solutions: displaySolutions
}
