use std::f64::EPSILON;

fn sqrt(num: f64) -> f64 {
    let mut y: f64 = num;
    let x2: f64 = y * 0.5;
    let i: *mut u64 = unsafe {
        &mut *(&mut y as *mut f64 as *mut u64)
    };

    unsafe { *i = 0x5fe6eb50c7b537a9 - (*i >> 1) };
    y = unsafe {
        *(i as *mut f64)
    };
    for _ in 0..6 {
        y = y * (1.5 - (x2 * y * y));
    }

    1.0 / y
}

pub struct Polynomial {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

pub struct Complex {
    pub a: f64,
    pub b: f64,
}

impl Polynomial {

    pub fn new() -> Polynomial {
        Polynomial { a: 0.0, b: 0.0, c: 0.0}
    }
    
    pub fn delta(&self) -> f64 {
        self.b * self.b - 4.0 * self.a * self.c 
    }

    pub fn degree(&self) -> i64 {
        let degree: i64;

        if self.a > EPSILON || self.a < -EPSILON {
            degree = 2;
        } else if self.b > EPSILON || self.b < -EPSILON {
            degree = 1;
        } else {
            degree = 0;
        }

        degree
    }

    fn solve_1(&self) -> Vec<Complex> {
        let mut res = Vec::<Complex>::new();

        res.push(Complex {
            a: (- self.c) / self.b,
            b: 0.0,
        });
        res
    }

    fn solve_2(&self) -> Vec<Complex> {
        let delta = self.delta();
        let mut res = Vec::<Complex>::new();

        if delta > EPSILON // Two solutions
        {
            res.push(Complex {
                a: (- self.b - sqrt(delta)) / (2.0 * self.a),
                b: 0.0,
            });
            res.push(Complex {
                a: (- self.b + sqrt(delta)) / (2.0 * self.a),
                b: 0.0,
            });
        }
        else if delta < -EPSILON  // Complex solutions
        {
            res.push(Complex {
                a: (- self.b) / (2.0 * self.a),
                b: (- sqrt(- delta)) / (2.0 * self.a),
            });
            res.push(Complex {
                a: (- self.b) / (2.0 * self.a),
                b: (sqrt(- delta)) / (2.0 * self.a),
            });
        }
        else // One solution (== 0)
        {
            res.push(Complex {
                a: (- self.b) / (2.0 * self.a),
                b: 0.0,
            });
        }
        res
    }

    pub fn solve(&self) -> Vec<Complex> {
        let degree = self.degree();

        match degree {
            2 => self.solve_2(),
            1 => self.solve_1(),
            _ => { Vec::<Complex>::new() }
        }
    }
}

pub fn reduce(left: &mut Polynomial, right: &mut Polynomial) {
    left.a = left.a - right.a;
    left.b = left.b - right.b;
    left.c = left.c - right.c;
    right.a = 0.0;
    right.b = 0.0;
    right.c = 0.0;
}
