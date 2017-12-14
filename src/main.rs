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
    let arg = parser::get_from_args();
    let parsed: (Option<(maths::Polynomial, maths::Polynomial)>, Option<String>);

    if arg.is_some() {
        parsed = parser::parse(&arg.unwrap());

        if parsed.1.is_some() {
            println!("Error: {}", parsed.1.unwrap());
        } else {
            let (mut left, mut right) = parsed.0.unwrap();

            println!("Success: {} = {}", left.to_string(), right.to_string());

            maths::reduce(&mut left, &mut right);
            println!("Reduced to :\n\t{} = 0", left.to_string());

            let solutions = left.solve();
     
            if solutions.len() == 0 {
                println!("No solution found.");
            } else {
                println!("Solutions:");
                for solution in solutions {
                    println!("{}", solution.to_string());
                }
            }
        }
    } else {
        println!("No equation string was given.");
    }
}
