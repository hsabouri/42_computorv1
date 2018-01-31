const Parser = require('./lib/parser')
const Display = require('./lib/display')

const [equation, left, right] = Parser.getEquation()

Display.unreduced(left, right)
