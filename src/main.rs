fn sqrt(num: f64) -> f64 {
    let mut y:  f64 = num;
    let x2:     f64 = y * 0.5;
    let i:  *mut u64 = unsafe {
        &mut *(&mut y as *mut f64 as *mut u64)
    };

    unsafe { *i = 0x5fe6eb50c7b537a9 - (*i >> 1) };
    y = unsafe {
        *(i as *mut f64)
    };
    for _ in 0..6 {
        y = y * (1.5 - (x2 * y * y));
            // Newton method 4 times for double precision
    }

    1.0 / y
}

pub struct Polynomial {
    a: f64,
    b: f64,
    c: f64,
}

pub struct Complex {
    a: f64,
    b: f64,
}

impl Polynomial {
    pub fn a(&self) -> &f64 {
        &self.a
    }

    pub fn b(&self) -> &f64 {
        &self.b
    }

    pub fn c(&self) -> &f64 {
        &self.c
    }

    pub fn new(a: f64, b: f64, c:f64) -> Polynomial {
        Polynomial { a: a, b: b, c: c }
    }

    pub fn delta(&self) -> f64 {
        self.b * self.b - 4.0 * self.a * self.c
    }

    pub fn solve(&self) -> Vec<Complex>  {
        let delta = self.delta();
        let mut res = Vec::<Complex>::new();

        if delta == 0.0 {
            res.push(Complex {
                a: (- self.b()) / (self.a() * 2.0),
                b: 0.0
            });
        } else if delta > 0.0 {
            res.push(Complex {
                a: (- self.b() - sqrt(delta)) / (2.0 * self.a()),
                b: 0.0
            });
            res.push(Complex {
                a: (- self.b() + sqrt(delta)) / (2.0 * self.a()),
                b: 0.0
            });
        }
        res
    }
}

fn main() {
    let poly = Polynomial { a: 10.0, b: -5.0, c: -1.0 };
    let solution = poly.solve();
    
    for x in solution {
        println!("{} + {}i", x.a, x.b);
    }
}
