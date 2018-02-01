const Parser = require('./lib/parser')
const Display = require('./lib/display')
const Maths = require('./lib/maths')
const error = require('./lib/error')

const _static_sortByOrder = equation => {
	let res = []

	for (let i in equation) {
		let toPush = 0

		for (let token in equation) {
			if (equation[token].order > equation[toPush].order)
				toPush = token
		}
		res = res.concat(equation.splice(toPush, 1))
	}
	res = res.concat(equation.splice(0, 1))
	return res
}

let equation = Parser.getEquation()
equation = _static_sortByOrder(equation)
const order = Maths.getOrder(equation)
const polynome = new Maths.polynome(equation)

Display.reduced(equation)
Display.degree(order)
if (order > 2)
	error("Provided equation's degree is not supported")

const solutions = polynome.solve()

if (order == 2) {
	Display.delta(polynome.getDelta())
}

Display.solutions(solutions)
