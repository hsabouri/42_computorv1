extern crate regex;
use self::regex::Regex;
use std::env;
use std::f64::EPSILON;
use maths;

pub struct Parser {
    left: maths::Polynomial,
    right: maths::Polynomial,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            left: maths::Polynomial { a: 0.0, b: 0.0, c: 0.0 },
            right: maths::Polynomial { a: 0.0, b: 0.0, c: 0.0 },
        }
    }

    fn parse_split(&self, arg: String) -> Option<(String, String)> {
        let reg = Regex::new(r"(?P<left>^[^=]+)=(?P<right>[^=]+$)").unwrap(); // Match sides of equation
        let captures = reg.captures(&arg);

        match captures.is_some() {
            true => {
                // Getting captured groups from regex 
                let left = captures.as_ref().unwrap().name("left").unwrap().as_str();
                let right = captures.as_ref().unwrap().name("right").unwrap().as_str();
                
                Some((String::from(left), String::from(right)))
            },
            false => None
        }
    }

    fn parse_part(&self, arg: String, poly: &mut maths::Polynomial) -> bool {
        let reg = Regex::new(r"(?P<val>^\s*[+-]?\s*\d+\.?\d*\s*)\*\s*x\s*\^\s*(?P<or>\d+\.?\d*$)");
        
        let captures = reg.captures(&arg);

        match captures.is_some() {
            true => {
                let order = captures.as_ref().unwrap().name("or").unwrap().as_str().parse::<f64>().unwrap();
                let value = captures.as_ref().unwrap().name("val").unwrap().as_str().parse::<f64>().unwrap();
                if order <= 2.0 + EPSILON && order >= 2.0 - EPSILON {
                    poly.a = value;
                    true
                } else if order <= 1.0 + EPSILON && order >= 1.0 - EPSILON {
                    poly.b = value;
                    true
                } else if order <= 0.0 + EPSILON && order >= 0.0 - EPSILON {
                    poly.c = value;
                    true
                } else {
                    println!("Equations of order {} are not supported", order);
                    false
                }
            },
            false => false
        }
    }

    fn parse_equation(&self, arg: String) -> bool {
        let sides: Option<(String, String)>;
        let reg = Regex::new(r"(?i:[-+]?\s*\d*\.?\d*\s*\*\s*x\s*\^\s*\d?\.?\d*)+").unwrap(); // Match parts of equation

        sides = self.parse_split(arg);
        match sides.is_some() {
            true => {
                for part in reg.captures_iter(&sides.as_ref().unwrap().0) {
                    self.parse_part(part.as_ref().unwrap().as_str(), "left");
                }
                for part in reg.captures_iter(&sides.as_ref().unwrap().1) {
                    self.parse_part(part.as_ref().unwrap().as_str(), "right");
                }
            },
            false => None
        }
    }

    pub fn parse(&self) {
        for (i, arg) in env::args().enumerate() {
            println!("arg-> {}", arg);
            if i > 0 {
                match arg.as_str() {
                    "-v" => {},
                    _ => self.parse_equation(arg),
                }
            }
        }
    }
}
