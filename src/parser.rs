use std::env;
use maths;

pub struct Parser {
    left: maths::Polynomial,
    right: maths::Polynomial,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            left: maths::Polynomial { a: 2.5, b: 8.0, c: 10.0 },
            right: maths::Polynomial { a: 0.0, b: 0.0, c: 0.0 },
        }
    }

    pub fn parse(&self) {
        for arg in env::args() {
            println!("arg-> {}", arg);
        }
    }
}
