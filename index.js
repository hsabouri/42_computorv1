const Parser = require('./lib/parser')
const Display = require('./lib/display')
const Maths = require('./lib/maths')
const error = require('./lib/error')

const equation = Parser.getEquation()

const order = Maths.getOrder(equation)
Display.reduced(equation)

console.log(`Polynomial degree: ${order}`)

if (order > 2)
	error("Provided equation's degree is not supported")

const polynome = new Maths.polynome(equation)

Display.delta(polynome.getDelta())

const solutions = polynome.solve()

Display.solutions(solutions)
