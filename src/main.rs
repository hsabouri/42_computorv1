mod maths;
mod parser;
use std::f64::EPSILON;

impl maths::Polynomial {
    pub fn to_string(&self) -> String {
        let mut res = String::new();

        if self.a > EPSILON && self.a < - EPSILON {
            res = format!("{}{}x^2 ", res, self.a);
        } else {
            res = format!("{}0x^2 ", res);
        }
        if self.b < -EPSILON {
            res = format!("{}- {}x ", res, -self.b);
        } else if self.b > EPSILON {
            res = format!("{}+ {}x ", res, self.b);
        } else {
            res = format!("{}+ 0x ", res);
        }
        if self.c < -EPSILON {
            res = format!("{}- {}", res, -self.c);
        } else if self.c > EPSILON {
            res = format!("{}+ {}", res, self.c);
        } else {
            res = format!("{}+ 0", res);
        }

        res
    }
}

impl maths::Complex {
    pub fn to_string(&self) -> String {
        let mut res = String::new();

        res = format!("{}{} ", res, self.a);
        if self.b < -EPSILON {
            res = format!("{}- {}i", res, -self.b);
        } else if self.b > EPSILON {
            res = format!("{}+ {}i", res, self.b);
        }

        res
    }
}

fn main() {
    let mut left = maths::Polynomial { a: 2.5, b: 8.0, c: 10.0 };
    let mut right = maths::Polynomial { a: 0.0, b: -1.0, c: -1.0 };
    let test = parser::Parser::new();
    let solutions: Vec<maths::Complex>;

    test.parse();
    println!("Parsed :\n\t{} = {}", left.to_string(), right.to_string());
    maths::reduce(&mut left, &mut right);

    solutions = left.solve();
    
    println!("Reduced to :\n\t{} = 0", left.to_string());
    println!("Solutions :");

    if solutions.len() == 0 {
        println!("No solution for this equation !");
    }

    for solution in solutions {
        println!("{}", solution.to_string());
    }
}
