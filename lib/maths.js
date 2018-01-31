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
}
