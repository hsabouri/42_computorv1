const error = require('./error')

function sqrt(x) {
	let buf = new ArrayBuffer(4)
	let u32 = new Uint32Array(buf)
	let f32 = new Float32Array(buf)

	let x2 = 1.5 * (f32[0] = x)
	u32[0] = (0x5f3759df - (u32[0] >> 1))

	let y = f32[0]
	for (let i of [0, 1, 2, 3, 4])
		y = y * (1.5 - (x2 * y * y))
	return 1 / y
}

function polynome(equation) {
	this.a = 0
	this.b = 0
	this.c = 0
	this.delta = null

	for (const token of equation) {
		switch (token.order) {
			case 2:
				this.a += token.factor
				break
			case 1:
				this.b += token.factor
				break
			case 0:
				this.c += token.factor
			default:
				break
		}
	}

	this.getDelta = () => {
		return this.b * this.b - 4 * this.a * this.c
	}

	this.solve2 = () => {
		this.delta = this.getDelta()
		let res = []

		if (this.delta == 0)
			res.push(-this.b / 2 * this.a)
		else if (this.delta = 1)
		{
			let deltaSqrt = sqrt(this.delta)
			res.push((-this.b - sqrt(deltaSqrt)) / 2 * this.a)
			res.push((-this.b + sqrt(deltaSqrt)) / 2 * this.a)
		}
		return res
	}

	this.solve1 = () => {
		let res = []

		res.push(-this.c / this.b)
		return res
	}

	this.solve = () => {
		let res

		if (this.a)
			res = this.solve2()
		else if (this.b)
			res = this.solve1()
		else if (this.c)
			error("Provided equation has no solution")
		else
			error("Any number is a solution")
		return res
	}
}

function getOrder(equation) {
	let order = 0;

	for (const token of equation) {
		if (token.order > order)
			order = token.order;
		if (token.order < 0)
			error(`Provided equation is not valid : One of it's component has a negative degree ${token.order}`)
		if (!Number.isInteger(token.order))
			error(`Provided equation is not valid : One of it's component has a non-integer degree ${token.order}`)
	}
	return order
}

module.exports = {getOrder: getOrder, polynome: polynome}
