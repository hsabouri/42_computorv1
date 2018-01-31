const XRegExp = require("xregexp")

const LOW_LEVEL_VERIFIER = XRegExp(`^((\\s*([-+]\\s*\\d+(\\.\\d+)?)?\\s*(\\*?\\s*([xX])\\s*(\\^\\s*([-+]?\\s*\\d+(\\.\\d+)?))?)?)*)(=(\\s*([-+]\\s*?\\d+(\\.\\d+)?)?\\s*(\\*?\\s*([xX])\\s*(\\^\\s*([-+]?\\s*\\d+(\\.\\d+)?))?)?)*)?$`)
const LOW_LEVEL_PARSING = XRegExp(`^\\s*(?<factor>[-+]?\\s*\\d+(.\\d+)?)?\\s*(?<first>\\*?\\s*([xX])\\s*(\\^\\s*(?<order>[-+]?\\s*\\d+(\\.\\d+)?))?)?\\s*$`)
const HIGH_LEVEL_LEXER = XRegExp(`^(?<left>[\\d+.\\-*\\s^xX]+)=(?<right>[\\d+.\\-*\\s^xX]+)$`)
const LOW_LEVEL_LEXER = XRegExp(`(\\s*([-+]?(\\s*\\d+(\\.\\d+)?\\s*\\*?)?\\s*([xX]\\s*(\\^\\s*[-+]?\\s*\\d+(\\.\\d+)?)?)?)\\s*)`)

function error(str) {
	console.error(str)
	process.exit()
}

function removeSpaces(str) {
	return str.replace(/\s/g, "")
}

function lowLevelVerifier(str) {
	const part = XRegExp.exec(str, LOW_LEVEL_VERIFIER)

	if (!part)
		return false
	else
		return true
}

function lowLevelParser(str) {
	const	parts = XRegExp.exec(str, LOW_LEVEL_PARSING)
	let		element = {
		factor: 0,
		order: 0
	}

	if (!parts)
		return element
	if (parts.factor)
		element.factor = Number(parts.factor)
	else if (parts.first)
		element.factor = 1
	if (parts.first && !parts.order)
		element.order = 1
	else if (parts.first && parts.order)
		element.order = Number(parts.order)
	return element
}

function highLevelLexer(str) {
	const	parts = XRegExp.exec(str, HIGH_LEVEL_LEXER)
	let		elements = []

	if (!parts)
		return elements
	if (parts.left)
		elements.push(parts.left)
	if (parts.right)
		elements.push(parts.right)
	else
		elements.push('0')
	return elements
}

function lowLevelLexer(str) {
	const	parts = XRegExp.match(str, LOW_LEVEL_LEXER, 'all')
	let		elements = []

	let		verifyStr = (str) => {
		charArray = str.split('')

		for (const c of charArray) {
			if (c != '\s' && c != '\t' && c != '\n' && c != '\v' && c != '\r' && c != '\0' && c != '-' && c != '+')
				return true
		}
		return false
	}
	
	for (const part of parts) {
		if (verifyStr(part)) {
			elements.push(part)
		}
	}
	return (elements)
}

function getEquation() {
	let		equationString = process.argv[2]

	// Verifying input
	if (!equationString) 
		error("Please provide an equation")

	if (!lowLevelVerifier(equationString)) 
		error("Provided equation is not valid")

	equationString = removeSpaces(equationString)

	let		sides = highLevelLexer(equationString)
	let		res = [{factor: 0.0, order: 1.0}]
	let		left = [{factor: 0.0, order: 1.0}]
	let		right = [{factor: 0.0, order: 1.0}]

	const	_static_searchOrder = (arr, token) => {
		for (const i in arr) {
			if (arr[i].order == token.order)
				return i
		}
		return -1
	}

	const	_static_addToken = (dst, toAdd) => {
		dst.factor += toAdd.factor
		return dst
	}
	
	const	_static_subToken = (dst, toSub) => {
		dst.factor -= toSub.factor
		return dst
	}

	// Parsing left part
	const partsLeft = lowLevelLexer(sides[0])

	for (let part of partsLeft) {
		const token = lowLevelParser(part)
		const orderIndex = _static_searchOrder(res, token) // Searching for existing order in the equation
		const leftOrderIndex = _static_searchOrder(left, token)

		if (token.factor != 0.0) {
			if (orderIndex >= 0) // If order already exist, just add the value
				res[orderIndex] = _static_addToken(res[orderIndex], token)
			else
				res.push(token)

			if (leftOrderIndex >= 0) // If order already exist, just add the value
				left[leftOrderIndex] = _static_addToken(left[leftOrderIndex], token)
			else
				left.push(token)
		}
	}
	
	// Parsing right part and substracting it to left part
	const partsRight = lowLevelLexer(sides[1])

	for (let part of partsRight) {
		const token = lowLevelParser(part)
		const orderIndex = _static_searchOrder(res, token)
		const rightOrderIndex = _static_searchOrder(right, token)

		if (token.factor != 0.0) {
			if (orderIndex >= 0) // If order already exist, just substract the value
				res[orderIndex] = _static_subToken(res[orderIndex], token)
			else
				res.push(token)

			if (rightOrderIndex >= 0) // If order already exist, just add the value
				right[rightOrderIndex] = _static_addToken(right[rightOrderIndex], token)
			else
				right.push(token)
		}
	}

	// Removing potential null values
	for (let i in res) {
		if (!res[i].factor)
			res.splice(i, 1)
	}

	return [res, left, right]
}

module.exports = {getEquation: getEquation}
