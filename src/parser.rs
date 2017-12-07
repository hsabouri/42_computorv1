extern crate regex;
use self::regex::Regex;
use std::io::{self, Write};
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

    fn parse_equation(&self, arg: String) {
        let sides: Option<(String, String)>;
        let reg = Regex::new(r"(?i:[-+]?\s*\d*\.?\d*\s*\*\s*x\s*\^\s*\d?\.?\d*)+").unwrap(); // Match parts of equation

        sides = self.parse_split(arg);
        match sides.is_some() {
            true => {
                for part in reg.captures_iter(&sides.as_ref().unwrap().0) {
                    println!("Found {}", &part[0]);
                }
            },
            false => println!("Can't parse equation !")
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
