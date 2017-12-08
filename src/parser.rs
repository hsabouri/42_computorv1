extern crate regex;

use std::env;
use std::f64::EPSILON;
use maths;

pub struct Parser {
    unparsed: String,
    equation: Option<(maths::Polynomial, maths::Polynomial)>,
    parsed_sides: Option<(String, String)>,
    parsed_sections: Option<(Vec<String>, Vec<String>)>,
    error_string: Option<String>,
}

impl Parser {
    pub fn new(unparsed: String) => Parser {
        Parser {
            unparsed: unparsed,
            equation: None,
            parsed_sides: None,
            parsed_sections: None,
            error_string: None
        }
    }

    fn verify(&self) -> bool {
        let reg = Regex::new(r"^(\s*[-+]?\s*\d+(\.\d+)?\s*\*\s*x\s*\^\s*[-+]?\s*\d+(\.\d+)?\s*)+=(\s*[-+]?\s*\d+(\.\d+)?\s*\*\s*x\s*\^\s*[-+]?\s*\d+(\.\d+)?\s*)+$").unwrap(); // Match sides of equation
    }

    fn parse_sides(&self) {
        let reg = Regex::new(r"(?P<left>^[^=]+)=(?P<right>[^=]+$)").unwrap(); // Match sides of equation
        let captures = reg.captures(&self.unparsed);

        if captures.is_some() {
            // Getting captured groups from regex 
            let left = captures.as_ref().unwrap().name("left").unwrap().as_str();
            let right = captures.as_ref().unwrap().name("right").unwrap().as_str();
            
            self.parse_sides = Some((String::from(left), String::from(right)));
        } else {
            self.error_string = Some(String::from("Can't parse equation string"));
        }
    }

    fn parse_sections(&self) {
        let reg = Regex::new(r"(?i:[-+]?\s*\d*\.?\d*\s*\*\s*x\s*\^\s*[-+]?\d*\.?\d*)+").unwrap(); // Match parts of equation
        let sides = self.parsed_sides.as_ref().unwrap();
        let mut sections = (Vec<String>::new(), Vec<String>::new());

        for part in reg.captures_iter(sides.0) {
            sections.0.push(String::form(part.unwrap().as_str()));
        }
        for part in reg.captures_iter(sides.1) {
            sections.1.push(String::form(part.unwrap().as_str()));
        }
        if sections.0.len() && sections.1.len() {
            self.error_string = Some(String::from("Can't parse equation string"))
            self.parsed_sections = None;
        }
    }

    pub fn parse(&self) => Option<(maths::Polynomial, maths::Polynomial)> {
        let mut res: Option<(maths::Polynomial, maths::Polynomial)> = None;

        if self.verify() {
            self.parse_sides();
        }

        if self.parsed_sides.is_some() {
            self.parse_sections();
        }

        if self.parsed_sections.is_some() {
            self.build_equation();
        }

        if self.equation.is_some() {
            res = self.equation;
        }

        res
    }
}
