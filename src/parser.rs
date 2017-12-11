extern crate regex;
use self::regex::Regex;

use std::env;
use std::f64::EPSILON;
use maths;

fn unspace(src: String) -> String {
    String::from(
        src.split_whitespace()
           .fold(String::new(), | s, i_s | format!("{}{}", s, i_s))
    )
}

pub struct Parser {
    unparsed: String,
    equation: Option<(maths::Polynomial, maths::Polynomial)>,
    parsed_sides: Option<(String, String)>,
    parsed_sections: Option<(Vec<String>, Vec<String>)>,
    error_string: Option<String>,
}

impl Parser {
    pub fn new(unparsed: String) -> Parser {
        Parser {
            unparsed: unparsed,
            equation: None,
            parsed_sides: None,
            parsed_sections: None,
            error_string: None
        }
    }

    fn verify(&self) -> bool {
        let reg = Regex::new(r"^(\s*[-+]?\s*\d+(\.\d+)?\s*\*\s*x[xX]\s*\^\s*[-+]?\s*\d+(\.\d+)?\s*)+=(\s*[-+]?\s*\d+(\.\d+)?\s*\*\s*x[xX]\s*\^\s*[-+]?\s*\d+(\.\d+)?\s*)+$").unwrap(); // Match sides of equation
        reg.is_match(&self.unparsed)
    }

    fn parse_sides(&mut self) {
        let reg = Regex::new(r"(?P<left>^[^=]+)=(?P<right>[^=]+$)").unwrap(); // Match sides of equation
        let captures = reg.captures(&self.unparsed);

        if captures.is_some() {
            // Getting captured groups from regex 
            let left = captures.as_ref().unwrap().name("left").unwrap().as_str();
            let right = captures.as_ref().unwrap().name("right").unwrap().as_str();
            
            self.parsed_sides = Some((String::from(left), String::from(right)));
        } else {
            self.error_string = Some(String::from("Can't parse equation string"));
        }
    }

    fn parse_sections(&mut self) {
        let reg = Regex::new(r"(?i:[-+]?\s*\d*\.?\d*\s*\*\s*[xX]\s*\^\s*[-+]?\d*\.?\d*)+").unwrap(); // Match parts of equation
        let sides = self.parsed_sides.as_ref().unwrap();
        let mut sections = (Vec::<String>::new(), Vec::<String>::new());

        for part in reg.captures_iter(&sides.0) {
            sections.0.push(String::from(&part[1]));
        }
        for part in reg.captures_iter(&sides.1) {
            sections.1.push(String::from(&part[1]));
        }
        if sections.0.len() > 0 && sections.1.len() > 0 {
            self.error_string = Some(String::from("Can't parse equation string"));
            self.parsed_sections = None;
        }
    }

    fn build_equation(&mut self) {
        let reg = Regex::new(r"^\s*(?P<a>[-+]?\s*\d+(\.\d+)?)\s*\*\s*[xX]\s*\^\s*(?P<b>[+-]?\s*\d+(\.\d+)?)$").unwrap();
        let mut error: bool = false;
        let mut equation = (maths::Polynomial::new(), maths::Polynomial::new());

        for part in self.parsed_sections.as_ref().unwrap().0.iter() {
            let captures = reg.captures(&part);

            if !captures.is_some() {
                self.equation = None;
                error = true;
                break;
            } else {
                let a = unspace(String::from(captures.as_ref().unwrap().name("a").unwrap().as_str()));
                let b = unspace(String::from(captures.as_ref().unwrap().name("b").unwrap().as_str()));

                println!("{} {}", a, b);
            }
        }
        if !error {
            // Add for loop
        }
    }

    pub fn parse(&mut self) -> &Option<(maths::Polynomial, maths::Polynomial)> {
        if self.verify() {
            self.parse_sides();
        }

        if self.parsed_sides.is_some() {
            self.parse_sections();
        }

        if self.parsed_sections.is_some() {
            self.build_equation();
        }

        &self.equation
    }
}
