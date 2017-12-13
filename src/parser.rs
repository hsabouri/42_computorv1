use std::env;
use std::f64::EPSILON;

extern crate regex;
use self::regex::Regex;

use maths;

const VERIFY_REGEX: &str = r"^(\s*[-+]?\s*\d+(\.\d+)?\s*\*\s*[xX]\s*\^\s*[-+]?\s*\d+(\.\d+)?\s*)+=(\s*[-+]?\s*\d+(\.\d+)?\s*\*\s*[xX]\s*\^\s*[-+]?\s*\d+(\.\d+)?\s*)+$";
const SIDES_REGEX: &str = r"(?P<left>^[^=]+)=(?P<right>[^=]+$)";
const SECTIONS_REGEX: &str = r"(?i:[-+]?\s*\d*\.?\d*\s*\*\s*[xX]\s*\^\s*[-+]?\d*\.?\d*)+";
const BUILD_REGEX: &str = r"^\s*(?P<a>[-+]?\s*\d+(\.\d+)?)\s*\*\s*[xX]\s*\^\s*(?P<b>[+-]?\s*\d+(\.\d+)?)$";

fn unspace(src: String) -> String {
    String::from(
        src.split_whitespace()
           .fold(String::new(), | s, i_s | format!("{}{}", s, i_s))
    )
}

fn verify(arg: &String) -> bool {
    let reg = Regex::new(VERIFY_REGEX).unwrap();
    reg.is_match(arg)
}

fn parse_sides(arg: &String) -> Option<(String, String)> {
    let reg = Regex::new(SIDES_REGEX).unwrap();
    let captures = reg.captures(arg);
    let ret: Option<(String, String)>;

    if captures.is_some() {
        let left = captures.as_ref().unwrap().name("left").unwrap().as_str();
        let right = captures.as_ref().unwrap().name("right").unwrap().as_str();

        ret = Some((String::from(left), String::from(right)));
    } else {
        ret = None;
    }

    ret
}

fn parse_sections(arg: &(String, String)) -> Option<(Vec<String>, Vec<String>)> {
    let reg = Regex::new(SECTIONS_REGEX).unwrap();
    let ret: Option<(Vec<String>, Vec<String>)>;
    let mut sections = (Vec::<String>::new(), Vec::<String>::new());

    for part in reg.captures_iter(&arg.0) {
        sections.0.push(String::from(&part[0]));
    }
    for part in reg.captures_iter(&arg.1) {
        sections.1.push(String::from(&part[0]));
    }
    if sections.0.len() == 0 || sections.1.len() == 0 {
        ret = None;
    } else {
        ret = Some(sections);
    }
    ret
}

fn is_valid_order(number: f64) -> bool {
    if number <= 0 + EPSILON && number >= 0 - EPSILON {
        true
    } else if number <= 1 + EPSILON && number >= 1 - EPSILON {
        true
    } else if number <= 2 + EPSILON && number >= 2 - EPSILON {
        true
    } else {
        false
    }
}

fn build_equation(sections: (Vec::<String>, Vec::<String>)) -> Option<(maths::Polynomial, maths::Polynomial)> {
    let reg = Regex::new(BUILD_REGEX).unwrap();
    let mut error: bool = false;
    let mut ret = (maths::Polynomial::new(), maths::Polynomial::new());

    for part in sections.0.iter() {
        let captures = reg.captures(&part);

        if !captures.is_some() {
            error = true;
            break;
        } else {
            let a = unspace(String::from(captures.as_ref().unwrap().name("a").unwrap().as_str()));
            let b = unspace(String::from(captures.as_ref().unwrap().name("b").unwrap().as_str()));

            // CODE HERE
        }
    }
}

pub fn parse(arg: &String) -> (Option<(maths::Polynomial, maths::Polynomial)>, Option<String>) {
    /*
    let mut equation: Option<(maths::Polynomial, maths::Polynomial)> = None;
    let mut error: Option<String> = None;
    */
    let mut ret: (Option<(maths::Polynomial, maths::Polynomial)>, Option<String>) = (None, None);
    let mut sides: Option<(String, String)> = None;
    let mut sections: Option<(Vec<String>, Vec<String>)> = None;

    if verify(arg) {
        sides = parse_sides(arg);
    } else {
        ret = (None, Some(String::from("Can't parse Equation")));
    }

    if sides.is_some() {
        sections = parse_sections(&sides.unwrap());
    } else {
        ret = (None, Some(String::from("Can't parse Equation")));
    }

    if sections.is_some() {
        for section in &sections.as_ref().unwrap().0 {
            println!("left: {}", section);
        }
        for section in &sections.as_ref().unwrap().1 {
            println!("right: {}", section);
        }
    }

    ret
}

pub fn get_from_args() -> Option<String> {
    let mut equation: Option<String> = None;
    for arg in env::args() {
        println!("ARG => {}", &arg);
        match arg.find("/").is_some() {
            true => {
                println!("Matched path")
            },
            false => {
                equation = Some(arg.clone());
                break;
            }
        }
    }

    equation
}
