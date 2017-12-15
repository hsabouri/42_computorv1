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
    let ret: bool;

    if number <= 0.0 + EPSILON && number >= 0.0 - EPSILON {
        ret = true;
    } else if number <= 1.0 + EPSILON && number >= 1.0 - EPSILON {
        ret = true;
    } else if number <= 2.0 + EPSILON && number >= 2.0 - EPSILON {
        ret = true;
    } else {
        ret = false;
    }

    ret
}

fn update_polynomial(polynom: &mut maths::Polynomial, a: f64, b: i64) {
    match b {
        2 => {
            polynom.a += a;
        },
        1 => {
            polynom.b += a;
        },
        0 => {
            polynom.c += a;
        },
        _ => {}
    }
}

fn build_equation(sections: (Vec<String>, Vec<String>)) -> Option<(maths::Polynomial, maths::Polynomial)> {
    let reg = Regex::new(BUILD_REGEX).unwrap();
    let mut error: bool = false;
    let mut equation = (maths::Polynomial::new(), maths::Polynomial::new());
    let ret: Option<(maths::Polynomial, maths::Polynomial)>;

    for part in sections.0.iter() {
        let captures = reg.captures(&part);

        if !captures.is_some() {
            error = true;
            break;
        } else {
            let a = unspace(String::from(captures.as_ref().unwrap().name("a").unwrap().as_str())).parse::<f64>().unwrap();
            let b = unspace(String::from(captures.as_ref().unwrap().name("b").unwrap().as_str())).parse::<f64>().unwrap();

            if is_valid_order(b) {
                update_polynomial(&mut equation.0, a, b as i64);
            } else {
                error = true;
                break;
            }
        }
    }
    if !error {
        for part in sections.1.iter() {
            let captures = reg.captures(&part);

            if !captures.is_some() {
                error = true;
                break;
            } else {
                let a = unspace(String::from(captures.as_ref().unwrap().name("a").unwrap().as_str())).parse::<f64>().unwrap();
                let b = unspace(String::from(captures.as_ref().unwrap().name("b").unwrap().as_str())).parse::<f64>().unwrap();

                if is_valid_order(b) {
                    update_polynomial(&mut equation.1, a, b as i64);
                } else {
                    error = true;
                    break;
                }
            }
        }
    }

    if !error {
        ret = Some(equation);
    } else {
        ret = None;
    }
    ret
}

pub fn parse(arg: &String) -> (Option<(maths::Polynomial, maths::Polynomial)>, Option<String>) {
    let mut sides: Option<(String, String)> = None;
    let mut sections: Option<(Vec<String>, Vec<String>)> = None;
    let mut equation: Option<(maths::Polynomial, maths::Polynomial)> = None;
    let mut ret: (Option<(maths::Polynomial, maths::Polynomial)>, Option<String>) = (None, None);

    if verify(arg) {
        sides = parse_sides(arg);
    } else {
        ret = (None, Some(String::from("Can't parse Equation")));
    }

    if sides.is_some() && !ret.1.is_some() {
        sections = parse_sections(&sides.unwrap());
    } else {
        ret = (None, Some(String::from("Can't parse Equation")));
    }

    if sections.is_some() && !ret.1.is_some() {
        equation = build_equation(sections.unwrap());
    } else {
        ret = (None, Some(String::from("Can't parse Equation")));
    }

    if equation.is_some() && !ret.1.is_some() {
        ret = (equation, None);
    } else {
        ret = (None, Some(String::from("Only equation of 1st and 2nd order are supported.")));
    }

    ret
}

pub fn get_from_args() -> Option<String> {
    let mut equation: Option<String> = None;
    for arg in env::args() {
        match arg.find("/").is_some() {
            true => {},
            false => {
                equation = Some(arg.clone());
                break;
            }
        }
    }

    equation
}
