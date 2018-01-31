function displayUnreduced(left, right) {
	let strArray = left.map(e => `${e.factor} * x ^ ${e.order}`)

	strArray.push("=")
	strArray = strArray.concat(right.map(e => `${e.factor} * x ^ ${e.order}`))
	console.log(strArray.join(' '))
}

module.exports = {unreduced: displayUnreduced}
